//! The app's state: the shared service.

use studyvut_service::Service;

pub struct AppState {
    pub service: Service,
}
