use jwst_codec::{Awareness, AwarenessEvent};

use super::*;

impl Workspace {
    pub async fn subscribe_awareness(&mut self, f: impl Fn(&Awareness, AwarenessEvent) + Send + Sync + 'static) {
        self.awareness.write().unwrap().on_update(f);
    }

    pub fn subscribe_doc(&mut self, f: impl Fn(&[u8]) + Sync + Send + 'static) {
        self.doc.subscribe(f)
    }

    pub fn unsubscribe_all(&mut self) {
        self.doc.unsubscribe_all();
        self.awareness.write().unwrap().on_update(|_, _| {})
    }
}
