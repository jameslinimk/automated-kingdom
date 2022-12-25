use derive_new::new;
use uuid::Uuid;

#[derive(new)]
pub struct ServerGame {
    #[new(value = "Uuid::new_v4()")]
    pub id: Uuid,
}
impl ServerGame {}
