use std::collections::HashSet;

use oauth2::Scope;

/// Manage scopes to avoid duplicate when initialization many times by using HashSet
pub struct GGScopeManager {
    scopes: HashSet<Scope>,
}

/// Kind of scopes to accept application access to user's google information
pub enum ScopeKind {
    UserProfile,
    UserEmail,
}

impl GGScopeManager {
    pub fn new() -> Self {
        let scopes = HashSet::new();

        Self { scopes }
    }

    pub fn add(&mut self, scopes_kind: Vec<ScopeKind>) -> &mut Self {
        scopes_kind.into_iter().for_each(|scope_kind| {
            let scope = match scope_kind {
                ScopeKind::UserProfile => {
                    Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string())
                }
                ScopeKind::UserEmail => {
                    Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string())
                }
            };
            self.scopes.insert(scope);
        });

        self
    }

    pub fn get_all(&mut self) -> &HashSet<Scope> {
        &self.scopes
    }
}
