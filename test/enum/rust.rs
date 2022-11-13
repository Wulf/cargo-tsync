/// test/rust.rs
use tsync::tsync;

/// Variants should to discriminated unions
/// The last serde/attribute combo matching the tag should be taken
#[derive(Serialize, Deserialize)]
#[serde(tag = "somethingelse")]
#[serde(renameAll = "kebab-case")]
#[serde(tag = "last_precedent")]
#[tsync]
enum Message {
    /// Per Enum case Docs One
    UnitCaseLeft,
    /// Per Enum case Docs Two
    RequestLongTake {
        id: String,
        method: String,
        params: i32,
    },
    Response {
        id: String,
        result: NaiveDateTime,
    },
}

/// The default enum conversion uses external tagging
#[tsync]
enum ExternalMessage {
    /// Per Enum case Docs One
    UnitCaseLeft,
    /// Per Enum case Docs Two
    RequestLongTake {
        id: String,
        method: String,
        params: i32,
    },
    Response {
        id: String,
        result: NaiveDateTime,
    },
}

/// All Unit Enums go to union of constant strings
/// even if have explicit numeric annotations
/// There is no case renaming on default
#[tsync]
enum Animal {
    Dog,
    Cat,
}
#[tsync]
#[serde(renameAll = "snake_case")]
enum AnimalTwo {
    DogLongExtra = 2,
    Cat,
}

/// Integer enums should follow rust discrimination if literals (doesn't evaluate expression)
#[derive(Serialize_repr)]
#[tsync]
enum Foo {
    Bar,       // 0
    Baz = 123, // 123
    Quux,      // 124
}
