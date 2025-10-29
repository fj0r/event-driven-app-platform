use super::config::Settings;
use axum::extract::FromRef;
use kafka::Created;
use message::{
    ChatMessage,
    session::{Session, SessionCount},
};
use serde_json::{Map, Value};
use std::fmt::Debug;
use std::sync::Arc;
use std::{
    collections::{
        HashMap,
        hash_map::{Entry, Iter},
    },
    ops::Deref,
};
use time::OffsetDateTime;
use tokio::sync::{RwLock, mpsc::UnboundedSender};

#[derive(Clone, Debug)]
pub struct SessionManager<T> {
    map: HashMap<Session, T>,
}

impl<'a, T> IntoIterator for &'a SessionManager<T> {
    type Item = (&'a Session, &'a T);
    type IntoIter = Iter<'a, Session, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<T> SessionManager<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, k: &Session) -> Option<&T> {
        self.map.get(k)
    }

    pub fn insert(&mut self, k: Session, v: T) -> Option<T> {
        self.map.insert(k, v)
    }

    pub fn remove(&mut self, k: &Session) -> Option<T> {
        self.map.remove(k)
    }

    pub fn contains_key(&self, k: &Session) -> bool {
        self.map.contains_key(k)
    }

    pub fn entry(&mut self, k: Session) -> Entry<'_, Session, T> {
        self.map.entry(k)
    }

    pub fn keys(&self) -> Vec<&Session> {
        self.map.keys().collect()
    }
}

pub type Arw<T> = Arc<RwLock<T>>;

#[derive(Debug, Clone)]
pub struct Shared<T> {
    pub session: Arw<SessionManager<T>>,
    pub count: Arw<SessionCount>,
    pub settings: Arw<Settings>,
}

impl<T: Clone> FromRef<Shared<T>> for Arw<SessionManager<T>> {
    fn from_ref(input: &Shared<T>) -> Self {
        input.session.clone()
    }
}

impl<T> FromRef<Shared<T>> for Arw<SessionCount> {
    fn from_ref(input: &Shared<T>) -> Self {
        input.count.clone()
    }
}

impl<T> FromRef<Shared<T>> for Arw<Settings> {
    fn from_ref(input: &Shared<T>) -> Self {
        input.settings.clone()
    }
}

impl<T> Shared<T> {
    pub fn new(settings: Arw<Settings>) -> Self {
        Shared {
            session: Arc::new(RwLock::new(SessionManager::new())),
            count: Arc::new(RwLock::new(SessionCount::default())),
            settings,
        }
    }
}

pub type Info = Map<String, Value>;

use tokio_util::sync::CancellationToken;
#[derive(Debug, Clone)]
pub struct Client<T> {
    pub sender: T,
    pub cancel: CancellationToken,
    //pub last_activity: OffsetDateTime,
    pub created: OffsetDateTime,
    pub info: Info,
}

impl<T> Deref for Client<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

pub type Sender = UnboundedSender<ChatMessage<Created>>;

pub type Arwsc<T> = Arc<RwLock<SessionManager<Client<T>>>>;
pub type StateChat<T> = Shared<Client<T>>;
