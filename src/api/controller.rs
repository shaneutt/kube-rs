use crate::api::informer::Informer;
use crate::{
    api::{
        resource::{KubeObject},
        Api,
    },
    client::APIClient,
};



use serde::de::DeserializeOwned;



#[derive(Clone)]
pub struct Controller<K>
where
    K: Clone + DeserializeOwned + KubeObject,
{
    client: APIClient,
    informer: Informer<K>,
}

impl<K> Controller<K>
where
    K: Clone + DeserializeOwned + KubeObject,
{
    /// Create a reflector with a kube client on a kube resource
    pub fn new(r: Api<K>) -> Self {
        Controller {
            client: r.client,
            // TODO: callback fn to reconcile
            // needs to just call it with "name" + namespace
            // TODO: informer init should always be from zero
            informer: Informer::new(r.api).init_from("0".into()),
        }
    }
}

impl<K> Controller<K>
where
    K: Clone + DeserializeOwned + KubeObject,
{
    /// Initialize
    pub fn init(self) -> Self {
        info!("Starting Controller for {:?}", self.resource);
        let r = self.resource.clone();
        let clone = self.clone();
        //futures::executor::block_on( ?
        tokio::spawn(async move {
            loop {
                match clone.informer.poll().await {
                    Err(e) => {
                        error!("Informer desync: {}: {:?}", e, e);
                        clone.informer.init_from("0".into()),
                    },
                    Ok(ev) => {
                        WatchEvent::Added(o) => {
                            debug!("Added Event for {:?}: {}" r, o.metadata.name);
                            // TODO: callback
                        }
                        WatchEvent::Modified(o) => {
                            debug!("Modified Event for {:?}: {}" r, o.metadata.name);
                            // TODO: callback
                        }
                        WatchEvent::Deleted(o) => {
                            debug!("Deleted Event for {:?}: {}" r, o.metadata.name);
                            // TODO: callback
                        }
                        WatchEvent::Error(e) => {
                            warn!("Error event: {:?}", e);
                            // TODO: is this a re-poll? probably
                            clone.informer.init_from("0".into()),
                        }
                    }
                }
            }
        });
        self
    }
}
