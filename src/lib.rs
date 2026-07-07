//! Shared cross-component standards.
//!
//! Where `signal-frame` owns domain-free wire mechanics (headers, envelopes,
//! stream tokens), `signal-standard` owns the domain-free cross-component
//! vocabulary every component conforms to: the reconciled [`ComponentKind`]
//! roster, the [`Differentiator`], the [`AuthorizedObjectInterest`] lattice,
//! and the small embeddable [`ComponentClassification`] nameplate. Per Spirit
//! `eeeo`.
//!
//! Wire-only vocabulary emitted from `schema/lib.schema`. This module adds the
//! hand-written escape-hatch methods over the emitted types.

#[rustfmt::skip]
#[allow(clippy::large_enum_variant, dead_code, private_interfaces)]
pub mod schema;

pub use schema::lib::*;

impl ObjectDigest {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl SocketPath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl HostName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl NetworkPort {
    pub fn into_u16(self) -> u16 {
        self.into_payload() as u16
    }
}

impl NetworkEndpoint {
    pub fn new(host: HostName, port: NetworkPort) -> Self {
        Self {
            host_name: host,
            network_port: port,
        }
    }

    pub fn host(&self) -> &HostName {
        &self.host_name
    }

    pub fn port(&self) -> &NetworkPort {
        &self.network_port
    }
}

impl Differentiator {
    pub fn new(component: ComponentKind, kind: AuthorizedObjectKind) -> Self {
        Self {
            component_kind: component,
            authorized_object_kind: kind,
        }
    }

    pub fn component(&self) -> ComponentKind {
        self.component_kind
    }

    pub fn kind(&self) -> AuthorizedObjectKind {
        self.authorized_object_kind
    }
}

impl ComponentObjectInterest {
    pub fn new(component: ComponentKind, kind: AuthorizedObjectKind) -> Self {
        Self {
            component_kind: component,
            authorized_object_kind: kind,
        }
    }

    pub fn component(&self) -> ComponentKind {
        self.component_kind
    }

    pub fn kind(&self) -> AuthorizedObjectKind {
        self.authorized_object_kind
    }
}

impl AuthorizedObjectReference {
    pub fn new(component: ComponentKind, digest: ObjectDigest, kind: AuthorizedObjectKind) -> Self {
        Self {
            component_kind: component,
            object_digest: digest,
            authorized_object_kind: kind,
        }
    }

    pub fn component(&self) -> ComponentKind {
        self.component_kind
    }

    pub fn digest(&self) -> &ObjectDigest {
        &self.object_digest
    }

    pub fn kind(&self) -> AuthorizedObjectKind {
        self.authorized_object_kind
    }

    pub fn matches_interest(&self, interest: &AuthorizedObjectInterest) -> bool {
        match interest {
            AuthorizedObjectInterest::AnyAuthorizedObject => true,
            AuthorizedObjectInterest::Component(component) => self.component_kind == *component,
            AuthorizedObjectInterest::ObjectKind(kind) => self.authorized_object_kind == *kind,
            AuthorizedObjectInterest::ComponentObject(component_object) => {
                self.component_kind == component_object.component_kind
                    && self.authorized_object_kind == component_object.authorized_object_kind
            }
        }
    }
}

impl ComponentClassification {
    pub fn new(differentiator: Differentiator, advertises: AuthorizedObjectInterest) -> Self {
        Self {
            differentiator,
            authorized_object_interest: advertises,
        }
    }

    pub fn advertises(&self) -> &AuthorizedObjectInterest {
        &self.authorized_object_interest
    }

    /// The any-object interest: a component that classifies itself as
    /// interested in every authorized object regardless of kind or owner.
    pub fn over_any(differentiator: Differentiator) -> Self {
        Self::new(
            differentiator,
            AuthorizedObjectInterest::AnyAuthorizedObject,
        )
    }
}
