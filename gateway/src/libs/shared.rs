use super::config::Settings;
use super::message::ChatMessage;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::{
    collections::{
        HashMap,
        hash_map::{Entry, Iter},
    },
    ops::Deref,
};
use tokio::sync::{Mutex, MutexGuard, RwLock, mpsc::UnboundedSender};

pub type SessionCount = u128;
pub type SessionId = String;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub struct Session(pub SessionId);

impl From<&str> for Session {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Session> for Value {
    fn from(value: Session) -> Self {
        value.0.into()
    }
}

impl From<SessionCount> for Session {
    fn from(value: SessionCount) -> Self {
        Self(value.to_string())
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for Session {
    type Target = SessionId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
}

#[derive(Debug, Clone)]
pub struct SharedState<T>(Arc<Mutex<Shared<T>>>);

impl<T> SharedState<T> {
    pub fn new(settings: Arc<RwLock<Settings>>) -> Self {
        SharedState::<T>(Arc::new(Mutex::new(Shared::new(settings))))
    }

    pub async fn read(&self) -> MutexGuard<Shared<T>> {
        self.0.lock().await
    }

    pub async fn write(&self) -> MutexGuard<Shared<T>> {
        self.0.lock().await
    }
}

#[derive(Debug, Clone)]
pub struct Shared<T> {
    pub session: SessionManager<T>,
    pub count: SessionCount,
    pub settings: Arc<RwLock<Settings>>,
}

impl<T> Shared<T> {
    pub fn new(settings: Arc<RwLock<Settings>>) -> Self {
        Shared {
            session: SessionManager::new(),
            count: SessionCount::default(),
            settings,
        }
    }
}

pub type Info = Option<Map<String, Value>>;

#[derive(Debug, Clone)]
pub struct Client<T> {
    pub sender: T,
    pub info: Info,
}

impl<T> Deref for Client<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

pub type Sender = UnboundedSender<ChatMessage>;

pub type StateChat<T> = SharedState<Client<T>>;
