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
        Self { host, port }
    }
}

impl Differentiator {
    pub fn new(component: ComponentKind, kind: AuthorizedObjectKind) -> Self {
        Self { component, kind }
    }
}

impl ComponentObjectInterest {
    pub fn new(component: ComponentKind, kind: AuthorizedObjectKind) -> Self {
        Self { component, kind }
    }
}

impl AuthorizedObjectReference {
    pub fn new(component: ComponentKind, digest: ObjectDigest, kind: AuthorizedObjectKind) -> Self {
        Self {
            component,
            digest,
            kind,
        }
    }

    pub fn matches_interest(&self, interest: &AuthorizedObjectInterest) -> bool {
        match interest {
            AuthorizedObjectInterest::AnyAuthorizedObject => true,
            AuthorizedObjectInterest::Component(component) => self.component == *component,
            AuthorizedObjectInterest::ObjectKind(kind) => self.kind == *kind,
            AuthorizedObjectInterest::ComponentObject(component_object) => {
                self.component == component_object.component && self.kind == component_object.kind
            }
        }
    }
}

impl ComponentClassification {
    pub fn new(differentiator: Differentiator, advertises: AuthorizedObjectInterest) -> Self {
        Self {
            differentiator,
            advertises,
        }
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
