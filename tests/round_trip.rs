//! NOTA round-trip witnesses for the shared cross-component vocabulary.
//!
//! Every type encodes to NOTA, decodes back, and re-encodes to the same text —
//! the codec witness for a pure-vocabulary contract. Exhaustive over the
//! `ComponentKind` roster (all 14 zone members) and one fixture per other type.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_standard::{
    AuthorizedObjectInterest, AuthorizedObjectKind, AuthorizedObjectReference,
    ComponentClassification, ComponentKind, ComponentObjectInterest, Differentiator, HostName,
    NetworkEndpoint, NetworkPort, ObjectDigest, SocketPath, StandardSocket,
};

fn round_trip_nota<T>(value: T)
where
    T: NotaEncode + NotaDecode + Clone + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    let recovered = NotaSource::new(&encoded).parse::<T>().expect("decode nota");
    assert_eq!(recovered, value, "decoded value must equal the original");
    assert_eq!(
        recovered.to_nota(),
        encoded,
        "re-encoded text must equal the first encoding"
    );
}

#[test]
fn every_component_kind_round_trips() {
    let roster = [
        ComponentKind::Spirit,
        ComponentKind::Mind,
        ComponentKind::Criome,
        ComponentKind::Message,
        ComponentKind::Router,
        ComponentKind::Mirror,
        ComponentKind::Terminal,
        ComponentKind::Harness,
        ComponentKind::Agent,
        ComponentKind::System,
        ComponentKind::Introspect,
        ComponentKind::Orchestrate,
        ComponentKind::Lojix,
        ComponentKind::Persona,
    ];
    assert_eq!(roster.len(), 14, "the reconciled roster has 14 variants");
    for component in roster {
        round_trip_nota(component);
    }
}

#[test]
fn every_authorized_object_kind_round_trips() {
    for kind in [
        AuthorizedObjectKind::Operation,
        AuthorizedObjectKind::Contract,
        AuthorizedObjectKind::Agreement,
        AuthorizedObjectKind::Time,
    ] {
        round_trip_nota(kind);
    }
}

#[test]
fn differentiator_round_trips() {
    round_trip_nota(Differentiator::new(
        ComponentKind::Criome,
        AuthorizedObjectKind::Contract,
    ));
}

#[test]
fn component_object_interest_round_trips() {
    round_trip_nota(ComponentObjectInterest::new(
        ComponentKind::Router,
        AuthorizedObjectKind::Operation,
    ));
}

#[test]
fn authorized_object_interest_lattice_round_trips() {
    round_trip_nota(AuthorizedObjectInterest::AnyAuthorizedObject);
    round_trip_nota(AuthorizedObjectInterest::Component(ComponentKind::Spirit));
    round_trip_nota(AuthorizedObjectInterest::ObjectKind(
        AuthorizedObjectKind::Time,
    ));
    round_trip_nota(AuthorizedObjectInterest::ComponentObject(
        ComponentObjectInterest::new(ComponentKind::Mirror, AuthorizedObjectKind::Agreement),
    ));
}

#[test]
fn authorized_object_reference_round_trips() {
    round_trip_nota(AuthorizedObjectReference::new(
        ComponentKind::Lojix,
        ObjectDigest::new("blake3-digest-fixture"),
        AuthorizedObjectKind::Operation,
    ));
}

#[test]
fn authorized_object_reference_matches_interest_lattice() {
    let reference = AuthorizedObjectReference::new(
        ComponentKind::Criome,
        ObjectDigest::new("contract-digest-fixture"),
        AuthorizedObjectKind::Contract,
    );

    assert!(reference.matches_interest(&AuthorizedObjectInterest::AnyAuthorizedObject));
    assert!(reference.matches_interest(&AuthorizedObjectInterest::Component(
        ComponentKind::Criome,
    )));
    assert!(reference.matches_interest(&AuthorizedObjectInterest::ObjectKind(
        AuthorizedObjectKind::Contract,
    )));
    assert!(reference.matches_interest(&AuthorizedObjectInterest::ComponentObject(
        ComponentObjectInterest::new(ComponentKind::Criome, AuthorizedObjectKind::Contract),
    )));

    assert!(!reference.matches_interest(&AuthorizedObjectInterest::Component(
        ComponentKind::Router,
    )));
    assert!(!reference.matches_interest(&AuthorizedObjectInterest::ObjectKind(
        AuthorizedObjectKind::Time,
    )));
    assert!(!reference.matches_interest(&AuthorizedObjectInterest::ComponentObject(
        ComponentObjectInterest::new(ComponentKind::Criome, AuthorizedObjectKind::Time),
    )));
    assert!(!reference.matches_interest(&AuthorizedObjectInterest::ComponentObject(
        ComponentObjectInterest::new(ComponentKind::Router, AuthorizedObjectKind::Contract),
    )));
}

#[test]
fn standard_socket_round_trips() {
    let socket_path = SocketPath::new("/run/user/1000/criome.socket");
    assert_eq!(socket_path.as_str(), "/run/user/1000/criome.socket");
    round_trip_nota(StandardSocket::UnixSocket(socket_path));

    let endpoint = NetworkEndpoint::new(
        HostName::new("prometheus.goldragon.criome"),
        NetworkPort::new(7474),
    );
    assert_eq!(endpoint.host.as_str(), "prometheus.goldragon.criome");
    assert_eq!(endpoint.port.clone().into_u16(), 7474);
    round_trip_nota(StandardSocket::NetworkSocket(endpoint));
}

#[test]
fn component_classification_round_trips() {
    round_trip_nota(ComponentClassification::new(
        Differentiator::new(ComponentKind::Agent, AuthorizedObjectKind::Contract),
        AuthorizedObjectInterest::Component(ComponentKind::Criome),
    ));
    round_trip_nota(ComponentClassification::over_any(Differentiator::new(
        ComponentKind::Spirit,
        AuthorizedObjectKind::Time,
    )));
}
