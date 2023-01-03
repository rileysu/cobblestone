use pin_project_lite::pin_project;
use tokio::io::AsyncRead;

pin_project! {
    #[derive(Debug)]
    pub struct EncryptedReader<R: AsyncRead> {
        #[pin]
        inner: R,
    }
}

#[derive(Debug)]
pub struct EncryptedWriter;

impl<R: AsyncRead> AsyncRead for EncryptedReader<R> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let me = self.project();

        let mut unencrypted_buf_mat = vec![0u8; buf.capacity()];
        let mut unencrypted_buf = tokio::io::ReadBuf::new(&mut unencrypted_buf_mat);

        let _res = me.inner.poll_read(cx, &mut unencrypted_buf)?;

        todo!()
    }
}
