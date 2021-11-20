// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Service register output structure
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RegisterServiceOutput {
    /// Id of the service that will be registered
    pub id: std::option::Option<std::string::String>,
    /// Name of the service that will be registered
    pub name: std::option::Option<std::string::String>,
}
impl RegisterServiceOutput {
    /// Id of the service that will be registered
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// Name of the service that will be registered
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl std::fmt::Debug for RegisterServiceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RegisterServiceOutput");
        formatter.field("id", &self.id);
        formatter.field("name", &self.name);
        formatter.finish()
    }
}
/// See [`RegisterServiceOutput`](crate::output::RegisterServiceOutput)
pub mod register_service_output {
    /// A builder for [`RegisterServiceOutput`](crate::output::RegisterServiceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Id of the service that will be registered
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// Id of the service that will be registered
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// Name of the service that will be registered
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// Name of the service that will be registered
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Consumes the builder and constructs a [`RegisterServiceOutput`](crate::output::RegisterServiceOutput)
        pub fn build(self) -> crate::output::RegisterServiceOutput {
            crate::output::RegisterServiceOutput {
                id: self.id,
                name: self.name,
            }
        }
    }
}
impl RegisterServiceOutput {
    /// Creates a new builder-style object to manufacture [`RegisterServiceOutput`](crate::output::RegisterServiceOutput)
    pub fn builder() -> crate::output::register_service_output::Builder {
        crate::output::register_service_output::Builder::default()
    }
}

/// Service healthcheck input structure
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct HealthcheckOutput {}
impl std::fmt::Debug for HealthcheckOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HealthcheckOutput");
        formatter.finish()
    }
}
/// See [`HealthcheckOutput`](crate::output::HealthcheckOutput)
pub mod healthcheck_output {
    /// A builder for [`HealthcheckOutput`](crate::output::HealthcheckOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`HealthcheckOutput`](crate::output::HealthcheckOutput)
        pub fn build(self) -> crate::output::HealthcheckOutput {
            crate::output::HealthcheckOutput {}
        }
    }
}
impl HealthcheckOutput {
    /// Creates a new builder-style object to manufacture [`HealthcheckOutput`](crate::output::HealthcheckOutput)
    pub fn builder() -> crate::output::healthcheck_output::Builder {
        crate::output::healthcheck_output::Builder::default()
    }
}