use anyhow::Context;
use oso::{Oso, PolarClass};

#[derive(Clone, Copy, PolarClass)]
struct User {}

#[derive(Clone, Copy, PolarClass)]
struct Doc1 {
    #[polar(attribute)]
    id: u8,
}

#[derive(Clone, Copy, PolarClass)]
struct Doc2 {
    #[polar(attribute)]
    id: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut oso = Oso::new();

    oso.register_class(User::get_polar_class())?;
    oso.register_class(Doc1::get_polar_class())?;
    // Oops!  We forgot to register Doc2.

    // ... and we forgot its Resource block too
    oso.load_str(
        r#"
        actor User {}

        resource Doc1 {
            permissions = [ "edit" ];
        }

        has_permission(_actor: User, "edit", doc1: Doc1)
            if doc1.id = 1;

        allow(actor: Actor, action: String, resource: Resource) if
            has_permission(actor, action, resource);
    "#,
    ).context("loading policy file")?;

    let user = User {};
    let doc1 = Doc1 { id: 1};
    let doc2 = Doc2 { id: 2 };
    assert!(oso.is_allowed(user, "edit", doc1).unwrap());
    // Why isn't this an error?
    assert!(!oso.is_allowed(user, "edit", doc2).unwrap());
    Ok(())
}
