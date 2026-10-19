use crate::notifiable::Notifiable;

// TODO: Define a public unit struct called EmailAlert

// TODO: Define a public unit struct called SystemAlert

// TODO: Implement Notifiable for EmailAlert with an empty impl block
// (This will use the default implementation)

// TODO: Implement Notifiable for SystemAlert with an empty impl block
// (This will use the default implementation)
pub struct EmailAlert;
pub struct SystemAlert;

impl Notifiable for EmailAlert {}
impl Notifiable for SystemAlert {}