use std::pin::Pin;

use futures::Future;

pub async fn yield_now() {
    struct YieldNow ( bool ); 
    impl Future for YieldNow {
        type Output = (); 
        fn poll ( self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.get_mut().0 = true; 
                cx.waker().wake_by_ref(); 
                std::task::Poll::Pending
            }
        }
    } 
    YieldNow(false).await; 
}
