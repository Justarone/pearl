use crate::prelude::*;

const WOULDBLOCK_RETRY_INTERVAL_MS: u64 = 10;

#[derive(Debug, Clone)]
pub(crate) struct File {
    pub(crate) read_fd: Arc<StdFile>,
    pub(crate) write_fd: Arc<Mutex<TokioFile>>,
}

#[inline]
fn schedule_wake(waker: Waker) {
    tokio::spawn(async move {
        delay_for(Duration::from_millis(WOULDBLOCK_RETRY_INTERVAL_MS))
            .map(|_| waker.wake_by_ref())
            .await;
    });
}

impl AsyncRead for File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<IOResult<usize>> {
        let mut file_ref = Pin::get_ref(self.as_ref());
        let pinned_file_ref = Pin::new(&mut file_ref);
        pinned_file_ref.poll_read(cx, buf)
    }
}

fn warn_and_wake(waker: Waker) {
    warn!(
        "file read operation wouldblock or interrupted, retry in {}ms",
        WOULDBLOCK_RETRY_INTERVAL_MS
    );
    schedule_wake(waker);
}

impl AsyncRead for &File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<IOResult<usize>> {
        let mut file = self.read_fd.as_ref();
        match file.read(buf) {
            Err(ref e)
                if e.kind() == IOErrorKind::WouldBlock || e.kind() == IOErrorKind::Interrupted =>
            {
                warn_and_wake(cx.waker().clone());
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
            Ok(n) => Poll::Ready(Ok(n)),
        }
    }
}

impl AsyncWrite for File {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<IOResult<usize>> {
        let mut file = self.read_fd.as_ref();
        match file.write_all(buf) {
            Err(ref e)
                if e.kind() == IOErrorKind::WouldBlock || e.kind() == IOErrorKind::Interrupted =>
            {
                warn_and_wake(cx.waker().clone());
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
            Ok(_) => Poll::Ready(Ok(buf.len())),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        let mut file = self.read_fd.as_ref();
        match file.flush() {
            Err(ref e)
                if e.kind() == IOErrorKind::WouldBlock || e.kind() == IOErrorKind::Interrupted =>
            {
                warn_and_wake(cx.waker().clone());
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
            Ok(_) => Poll::Ready(Ok(())),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        self.poll_flush(cx)
    }
}

impl AsyncSeek for File {
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context<'_>, pos: SeekFrom) -> Poll<IOResult<u64>> {
        let mut file_ref = Pin::get_ref(self.as_ref());
        let pinned_file_ref = Pin::new(&mut file_ref);
        pinned_file_ref.poll_seek(cx, pos)
    }
}

impl AsyncSeek for &File {
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context<'_>, pos: SeekFrom) -> Poll<IOResult<u64>> {
        let mut file = self.read_fd.as_ref();
        match file.seek(pos) {
            Err(ref e)
                if e.kind() == IOErrorKind::WouldBlock || e.kind() == IOErrorKind::Interrupted =>
            {
                warn_and_wake(cx.waker().clone());
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
            Ok(n) => Poll::Ready(Ok(n)),
        }
    }
}

impl File {
    pub(crate) fn metadata(&self) -> IOResult<std::fs::Metadata> {
        self.read_fd.metadata()
    }

    pub(crate) async fn write_at(&self, buf: Vec<u8>, offset: u64) -> IOResult<usize> {
        let fd = self.read_fd.clone();
        let write_fut = WriteAt { fd, buf, offset };
        write_fut.await
    }

    pub(crate) async fn read_at(&self, len: usize, offset: u64) -> IOResult<Vec<u8>> {
        let read_fut = ReadAt {
            fd: self.read_fd.clone(),
            len,
            offset,
        };
        read_fut.await
    }

    pub(crate) async fn from_tokio_file(file: TokioFile) -> Self {
        todo!()
    }

    pub(crate) fn from_std_file(fd: StdFile) -> IOResult<Self> {
        let file = fd.try_clone()?;
        Ok(Self {
            read_fd: Arc::new(file),
            write_fd: Arc::new(Mutex::new(TokioFile::from_std(fd))),
        })
    }

    pub(crate) async fn fsync(&self) {
        todo!()
    }
}

struct WriteAt {
    fd: Arc<StdFile>,
    buf: Vec<u8>,
    offset: u64,
}

impl<'a> Future for WriteAt {
    type Output = IOResult<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.fd.write_at(&self.buf, self.offset) {
            Err(ref e)
                if e.kind() == IOErrorKind::WouldBlock || e.kind() == IOErrorKind::Interrupted =>
            {
                warn_and_wake(cx.waker().clone());
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
            Ok(_) => Poll::Ready(Ok(self.buf.len())),
        }
    }
}

#[derive(Debug)]
struct ReadAt {
    fd: Arc<StdFile>,
    len: usize,
    offset: u64,
}

impl Future for ReadAt {
    type Output = IOResult<Vec<u8>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        trace!("read at poll {:?}", self);
        let mut buf = vec![0; self.len];
        match self.fd.read_at(&mut buf, self.offset) {
            Err(ref e)
                if e.kind() == IOErrorKind::WouldBlock || e.kind() == IOErrorKind::Interrupted =>
            {
                warn_and_wake(cx.waker().clone());
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
            Ok(n) => Poll::Ready(Ok(buf[0..n].to_vec())),
        }
    }
}
