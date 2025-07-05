pub async fn logic(
    mqrx: Arc<Mutex<UnboundedReceiver<Envelope<Created>>>>,
    mqtx: Arc<Mutex<UnboundedSender<ParseCharError<Created>>>>,
    shared: ,
) {
    let shared = shared.clone();
    tokio::spawn(async move {
        let mut rx = mqrx.lock().await;

        while let Some(x) = rx.recv().await {
            if !x.receiver.is_empty() {
                let s = shared.session.write().await;
                for r in x.receiver {
                    if s.contains_key(&r) {
                        let s = s.get(&r)?;
                        let _ = s.send(x.message.clone());
                    }
                }
            }
        }
        Some(())
    });
}
