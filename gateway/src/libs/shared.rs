use super::config::Settings;
use kafka::Created;
use message::ChatMessage;
use axum::extract::FromRef;
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
use tokio::sync::{RwLock, mpsc::UnboundedSender};

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

pub type Sender = UnboundedSender<ChatMessage<Session, Created>>;

pub type Arwsc<T> = Arc<RwLock<SessionManager<Client<T>>>>;
pub type StateChat<T> = Shared<Client<T>>;
