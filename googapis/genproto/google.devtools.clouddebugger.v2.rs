/// Represents a message with parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatMessage {
    /// Format template for the message. The `format` uses placeholders `$0`,
    /// `$1`, etc. to reference parameters. `$$` can be used to denote the `$`
    /// character.
    ///
    /// Examples:
    ///
    /// *   `Failed to load '$0' which helps debug $1 the first time it
    ///     is loaded.  Again, $0 is very important.`
    /// *   `Please pay $$10 to use $0 instead of $1.`
    #[prost(string, tag = "1")]
    pub format: std::string::String,
    /// Optional parameters to be embedded into the message.
    #[prost(string, repeated, tag = "2")]
    pub parameters: ::std::vec::Vec<std::string::String>,
}
/// Represents a contextual status message.
/// The message can indicate an error or informational status, and refer to
/// specific parts of the containing object.
/// For example, the `Breakpoint.status` field can indicate an error referring
/// to the `BREAKPOINT_SOURCE_LOCATION` with the message `Location not found`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusMessage {
    /// Distinguishes errors from informational messages.
    #[prost(bool, tag = "1")]
    pub is_error: bool,
    /// Reference to which the message applies.
    #[prost(enumeration = "status_message::Reference", tag = "2")]
    pub refers_to: i32,
    /// Status message text.
    #[prost(message, optional, tag = "3")]
    pub description: ::std::option::Option<FormatMessage>,
}
pub mod status_message {
    /// Enumerates references to which the message applies.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reference {
        /// Status doesn't refer to any particular input.
        Unspecified = 0,
        /// Status applies to the breakpoint and is related to its location.
        BreakpointSourceLocation = 3,
        /// Status applies to the breakpoint and is related to its condition.
        BreakpointCondition = 4,
        /// Status applies to the breakpoint and is related to its expressions.
        BreakpointExpression = 7,
        /// Status applies to the breakpoint and is related to its age.
        BreakpointAge = 8,
        /// Status applies to the entire variable.
        VariableName = 5,
        /// Status applies to variable value (variable name is valid).
        VariableValue = 6,
    }
}
/// Represents a location in the source code.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceLocation {
    /// Path to the source file within the source context of the target binary.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Line inside the file. The first line in the file has the value `1`.
    #[prost(int32, tag = "2")]
    pub line: i32,
    /// Column within a line. The first column in a line as the value `1`.
    /// Agents that do not support setting breakpoints on specific columns ignore
    /// this field.
    #[prost(int32, tag = "3")]
    pub column: i32,
}
/// Represents a variable or an argument possibly of a compound object type.
/// Note how the following variables are represented:
///
/// 1) A simple variable:
///
///     int x = 5
///
///     { name: "x", value: "5", type: "int" }  // Captured variable
///
/// 2) A compound object:
///
///     struct T {
///         int m1;
///         int m2;
///     };
///     T x = { 3, 7 };
///
///     {  // Captured variable
///         name: "x",
///         type: "T",
///         members { name: "m1", value: "3", type: "int" },
///         members { name: "m2", value: "7", type: "int" }
///     }
///
/// 3) A pointer where the pointee was captured:
///
///     T x = { 3, 7 };
///     T* p = &x;
///
///     {   // Captured variable
///         name: "p",
///         type: "T*",
///         value: "0x00500500",
///         members { name: "m1", value: "3", type: "int" },
///         members { name: "m2", value: "7", type: "int" }
///     }
///
/// 4) A pointer where the pointee was not captured:
///
///     T* p = new T;
///
///     {   // Captured variable
///         name: "p",
///         type: "T*",
///         value: "0x00400400"
///         status { is_error: true, description { format: "unavailable" } }
///     }
///
/// The status should describe the reason for the missing value,
/// such as `<optimized out>`, `<inaccessible>`, `<pointers limit reached>`.
///
/// Note that a null pointer should not have members.
///
/// 5) An unnamed value:
///
///     int* p = new int(7);
///
///     {   // Captured variable
///         name: "p",
///         value: "0x00500500",
///         type: "int*",
///         members { value: "7", type: "int" } }
///
/// 6) An unnamed pointer where the pointee was not captured:
///
///     int* p = new int(7);
///     int** pp = &p;
///
///     {  // Captured variable
///         name: "pp",
///         value: "0x00500500",
///         type: "int**",
///         members {
///             value: "0x00400400",
///             type: "int*"
///             status {
///                 is_error: true,
///                 description: { format: "unavailable" } }
///             }
///         }
///     }
///
/// To optimize computation, memory and network traffic, variables that
/// repeat in the output multiple times can be stored once in a shared
/// variable table and be referenced using the `var_table_index` field.  The
/// variables stored in the shared table are nameless and are essentially
/// a partition of the complete variable. To reconstruct the complete
/// variable, merge the referencing variable with the referenced variable.
///
/// When using the shared variable table, the following variables:
///
///     T x = { 3, 7 };
///     T* p = &x;
///     T& r = x;
///
///     { name: "x", var_table_index: 3, type: "T" }  // Captured variables
///     { name: "p", value "0x00500500", type="T*", var_table_index: 3 }
///     { name: "r", type="T&", var_table_index: 3 }
///
///     {  // Shared variable table entry #3:
///         members { name: "m1", value: "3", type: "int" },
///         members { name: "m2", value: "7", type: "int" }
///     }
///
/// Note that the pointer address is stored with the referencing variable
/// and not with the referenced variable. This allows the referenced variable
/// to be shared between pointers and references.
///
/// The type field is optional. The debugger agent may or may not support it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    /// Name of the variable, if any.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Simple value of the variable.
    #[prost(string, tag = "2")]
    pub value: std::string::String,
    /// Variable type (e.g. `MyClass`). If the variable is split with
    /// `var_table_index`, `type` goes next to `value`. The interpretation of
    /// a type is agent specific. It is recommended to include the dynamic type
    /// rather than a static type of an object.
    #[prost(string, tag = "6")]
    pub r#type: std::string::String,
    /// Members contained or pointed to by the variable.
    #[prost(message, repeated, tag = "3")]
    pub members: ::std::vec::Vec<Variable>,
    /// Reference to a variable in the shared variable table. More than
    /// one variable can reference the same variable in the table. The
    /// `var_table_index` field is an index into `variable_table` in Breakpoint.
    #[prost(message, optional, tag = "4")]
    pub var_table_index: ::std::option::Option<i32>,
    /// Status associated with the variable. This field will usually stay
    /// unset. A status of a single variable only applies to that variable or
    /// expression. The rest of breakpoint data still remains valid. Variables
    /// might be reported in error state even when breakpoint is not in final
    /// state.
    ///
    /// The message may refer to variable name with `refers_to` set to
    /// `VARIABLE_NAME`. Alternatively `refers_to` will be set to `VARIABLE_VALUE`.
    /// In either case variable value and members will be unset.
    ///
    /// Example of error message applied to name: `Invalid expression syntax`.
    ///
    /// Example of information message applied to value: `Not captured`.
    ///
    /// Examples of error message applied to value:
    ///
    /// *   `Malformed string`,
    /// *   `Field f not found in class C`
    /// *   `Null pointer dereference`
    #[prost(message, optional, tag = "5")]
    pub status: ::std::option::Option<StatusMessage>,
}
/// Represents a stack frame context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackFrame {
    /// Demangled function name at the call site.
    #[prost(string, tag = "1")]
    pub function: std::string::String,
    /// Source location of the call site.
    #[prost(message, optional, tag = "2")]
    pub location: ::std::option::Option<SourceLocation>,
    /// Set of arguments passed to this function.
    /// Note that this might not be populated for all stack frames.
    #[prost(message, repeated, tag = "3")]
    pub arguments: ::std::vec::Vec<Variable>,
    /// Set of local variables at the stack frame location.
    /// Note that this might not be populated for all stack frames.
    #[prost(message, repeated, tag = "4")]
    pub locals: ::std::vec::Vec<Variable>,
}
/// Represents the breakpoint specification, status and results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Breakpoint {
    /// Breakpoint identifier, unique in the scope of the debuggee.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Action that the agent should perform when the code at the
    /// breakpoint location is hit.
    #[prost(enumeration = "breakpoint::Action", tag = "13")]
    pub action: i32,
    /// Breakpoint source location.
    #[prost(message, optional, tag = "2")]
    pub location: ::std::option::Option<SourceLocation>,
    /// Condition that triggers the breakpoint.
    /// The condition is a compound boolean expression composed using expressions
    /// in a programming language at the source location.
    #[prost(string, tag = "3")]
    pub condition: std::string::String,
    /// List of read-only expressions to evaluate at the breakpoint location.
    /// The expressions are composed using expressions in the programming language
    /// at the source location. If the breakpoint action is `LOG`, the evaluated
    /// expressions are included in log statements.
    #[prost(string, repeated, tag = "4")]
    pub expressions: ::std::vec::Vec<std::string::String>,
    /// Only relevant when action is `LOG`. Defines the message to log when
    /// the breakpoint hits. The message may include parameter placeholders `$0`,
    /// `$1`, etc. These placeholders are replaced with the evaluated value
    /// of the appropriate expression. Expressions not referenced in
    /// `log_message_format` are not logged.
    ///
    /// Example: `Message received, id = $0, count = $1` with
    /// `expressions` = `[ message.id, message.count ]`.
    #[prost(string, tag = "14")]
    pub log_message_format: std::string::String,
    /// Indicates the severity of the log. Only relevant when action is `LOG`.
    #[prost(enumeration = "breakpoint::LogLevel", tag = "15")]
    pub log_level: i32,
    /// When true, indicates that this is a final result and the
    /// breakpoint state will not change from here on.
    #[prost(bool, tag = "5")]
    pub is_final_state: bool,
    /// Time this breakpoint was created by the server in seconds resolution.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time this breakpoint was finalized as seen by the server in seconds
    /// resolution.
    #[prost(message, optional, tag = "12")]
    pub final_time: ::std::option::Option<::prost_types::Timestamp>,
    /// E-mail address of the user that created this breakpoint
    #[prost(string, tag = "16")]
    pub user_email: std::string::String,
    /// Breakpoint status.
    ///
    /// The status includes an error flag and a human readable message.
    /// This field is usually unset. The message can be either
    /// informational or an error message. Regardless, clients should always
    /// display the text message back to the user.
    ///
    /// Error status indicates complete failure of the breakpoint.
    ///
    /// Example (non-final state): `Still loading symbols...`
    ///
    /// Examples (final state):
    ///
    /// *   `Invalid line number` referring to location
    /// *   `Field f not found in class C` referring to condition
    #[prost(message, optional, tag = "10")]
    pub status: ::std::option::Option<StatusMessage>,
    /// The stack at breakpoint time, where stack_frames[0] represents the most
    /// recently entered function.
    #[prost(message, repeated, tag = "7")]
    pub stack_frames: ::std::vec::Vec<StackFrame>,
    /// Values of evaluated expressions at breakpoint time.
    /// The evaluated expressions appear in exactly the same order they
    /// are listed in the `expressions` field.
    /// The `name` field holds the original expression text, the `value` or
    /// `members` field holds the result of the evaluated expression.
    /// If the expression cannot be evaluated, the `status` inside the `Variable`
    /// will indicate an error and contain the error text.
    #[prost(message, repeated, tag = "8")]
    pub evaluated_expressions: ::std::vec::Vec<Variable>,
    /// The `variable_table` exists to aid with computation, memory and network
    /// traffic optimization.  It enables storing a variable once and reference
    /// it from multiple variables, including variables stored in the
    /// `variable_table` itself.
    /// For example, the same `this` object, which may appear at many levels of
    /// the stack, can have all of its data stored once in this table.  The
    /// stack frame variables then would hold only a reference to it.
    ///
    /// The variable `var_table_index` field is an index into this repeated field.
    /// The stored objects are nameless and get their name from the referencing
    /// variable. The effective variable is a merge of the referencing variable
    /// and the referenced variable.
    #[prost(message, repeated, tag = "9")]
    pub variable_table: ::std::vec::Vec<Variable>,
    /// A set of custom breakpoint properties, populated by the agent, to be
    /// displayed to the user.
    #[prost(map = "string, string", tag = "17")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
pub mod breakpoint {
    /// Actions that can be taken when a breakpoint hits.
    /// Agents should reject breakpoints with unsupported or unknown action values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// Capture stack frame and variables and update the breakpoint.
        /// The data is only captured once. After that the breakpoint is set
        /// in a final state.
        Capture = 0,
        /// Log each breakpoint hit. The breakpoint remains active until
        /// deleted or expired.
        Log = 1,
    }
    /// Log severity levels.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogLevel {
        /// Information log message.
        Info = 0,
        /// Warning log message.
        Warning = 1,
        /// Error log message.
        Error = 2,
    }
}
/// Represents the debugged application. The application may include one or more
/// replicated processes executing the same code. Each of these processes is
/// attached with a debugger agent, carrying out the debugging commands.
/// Agents attached to the same debuggee identify themselves as such by using
/// exactly the same Debuggee message value when registering.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Debuggee {
    /// Unique identifier for the debuggee generated by the controller service.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Project the debuggee is associated with.
    /// Use project number or id when registering a Google Cloud Platform project.
    #[prost(string, tag = "2")]
    pub project: std::string::String,
    /// Uniquifier to further distinguish the application.
    /// It is possible that different applications might have identical values in
    /// the debuggee message, thus, incorrectly identified as a single application
    /// by the Controller service. This field adds salt to further distinguish the
    /// application. Agents should consider seeding this field with value that
    /// identifies the code, binary, configuration and environment.
    #[prost(string, tag = "3")]
    pub uniquifier: std::string::String,
    /// Human readable description of the debuggee.
    /// Including a human-readable project name, environment name and version
    /// information is recommended.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
    /// If set to `true`, indicates that Controller service does not detect any
    /// activity from the debuggee agents and the application is possibly stopped.
    #[prost(bool, tag = "5")]
    pub is_inactive: bool,
    /// Version ID of the agent.
    /// Schema: `domain/language-platform/vmajor.minor` (for example
    /// `google.com/java-gcp/v1.1`).
    #[prost(string, tag = "6")]
    pub agent_version: std::string::String,
    /// If set to `true`, indicates that the agent should disable itself and
    /// detach from the debuggee.
    #[prost(bool, tag = "7")]
    pub is_disabled: bool,
    /// Human readable message to be displayed to the user about this debuggee.
    /// Absence of this field indicates no status. The message can be either
    /// informational or an error status.
    #[prost(message, optional, tag = "8")]
    pub status: ::std::option::Option<StatusMessage>,
    /// References to the locations and revisions of the source code used in the
    /// deployed application.
    #[prost(message, repeated, tag = "9")]
    pub source_contexts: ::std::vec::Vec<super::super::source::v1::SourceContext>,
    /// References to the locations and revisions of the source code used in the
    /// deployed application.
    #[prost(message, repeated, tag = "13")]
    pub ext_source_contexts: ::std::vec::Vec<super::super::source::v1::ExtendedSourceContext>,
    /// A set of custom debuggee properties, populated by the agent, to be
    /// displayed to the user.
    #[prost(map = "string, string", tag = "11")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Request to register a debuggee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDebuggeeRequest {
    /// Required. Debuggee information to register.
    /// The fields `project`, `uniquifier`, `description` and `agent_version`
    /// of the debuggee must be set.
    #[prost(message, optional, tag = "1")]
    pub debuggee: ::std::option::Option<Debuggee>,
}
/// Response for registering a debuggee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDebuggeeResponse {
    /// Debuggee resource.
    /// The field `id` is guaranteed to be set (in addition to the echoed fields).
    /// If the field `is_disabled` is set to `true`, the agent should disable
    /// itself by removing all breakpoints and detaching from the application.
    /// It should however continue to poll `RegisterDebuggee` until reenabled.
    #[prost(message, optional, tag = "1")]
    pub debuggee: ::std::option::Option<Debuggee>,
}
/// Request to list active breakpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActiveBreakpointsRequest {
    /// Required. Identifies the debuggee.
    #[prost(string, tag = "1")]
    pub debuggee_id: std::string::String,
    /// A token that, if specified, blocks the method call until the list
    /// of active breakpoints has changed, or a server-selected timeout has
    /// expired. The value should be set from the `next_wait_token` field in
    /// the last response. The initial value should be set to `"init"`.
    #[prost(string, tag = "2")]
    pub wait_token: std::string::String,
    /// If set to `true` (recommended), returns `google.rpc.Code.OK` status and
    /// sets the `wait_expired` response field to `true` when the server-selected
    /// timeout has expired.
    ///
    /// If set to `false` (deprecated), returns `google.rpc.Code.ABORTED` status
    /// when the server-selected timeout has expired.
    #[prost(bool, tag = "3")]
    pub success_on_timeout: bool,
}
/// Response for listing active breakpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActiveBreakpointsResponse {
    /// List of all active breakpoints.
    /// The fields `id` and `location` are guaranteed to be set on each breakpoint.
    #[prost(message, repeated, tag = "1")]
    pub breakpoints: ::std::vec::Vec<Breakpoint>,
    /// A token that can be used in the next method call to block until
    /// the list of breakpoints changes.
    #[prost(string, tag = "2")]
    pub next_wait_token: std::string::String,
    /// If set to `true`, indicates that there is no change to the
    /// list of active breakpoints and the server-selected timeout has expired.
    /// The `breakpoints` field would be empty and should be ignored.
    #[prost(bool, tag = "3")]
    pub wait_expired: bool,
}
/// Request to update an active breakpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActiveBreakpointRequest {
    /// Required. Identifies the debuggee being debugged.
    #[prost(string, tag = "1")]
    pub debuggee_id: std::string::String,
    /// Required. Updated breakpoint information.
    /// The field `id` must be set.
    /// The agent must echo all Breakpoint specification fields in the update.
    #[prost(message, optional, tag = "2")]
    pub breakpoint: ::std::option::Option<Breakpoint>,
}
/// Response for updating an active breakpoint.
/// The message is defined to allow future extensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActiveBreakpointResponse {}
#[doc = r" Generated client implementations."]
pub mod controller2_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Controller service provides the API for orchestrating a collection of"]
    #[doc = " debugger agents to perform debugging tasks. These agents are each attached"]
    #[doc = " to a process of an application which may include one or more replicas."]
    #[doc = ""]
    #[doc = " The debugger agents register with the Controller to identify the application"]
    #[doc = " being debugged, the Debuggee. All agents that register with the same data,"]
    #[doc = " represent the same Debuggee, and are assigned the same `debuggee_id`."]
    #[doc = ""]
    #[doc = " The debugger agents call the Controller to retrieve  the list of active"]
    #[doc = " Breakpoints. Agents with the same `debuggee_id` get the same breakpoints"]
    #[doc = " list. An agent that can fulfill the breakpoint request updates the"]
    #[doc = " Controller with the breakpoint result. The controller selects the first"]
    #[doc = " result received and discards the rest of the results."]
    #[doc = " Agents that poll again for active breakpoints will no longer have"]
    #[doc = " the completed breakpoint in the list and should remove that breakpoint from"]
    #[doc = " their attached process."]
    #[doc = ""]
    #[doc = " The Controller service does not provide a way to retrieve the results of"]
    #[doc = " a completed breakpoint. This functionality is available using the Debugger"]
    #[doc = " service."]
    pub struct Controller2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> Controller2Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Registers the debuggee with the controller service."]
        #[doc = ""]
        #[doc = " All agents attached to the same application must call this method with"]
        #[doc = " exactly the same request content to get back the same stable `debuggee_id`."]
        #[doc = " Agents should call this method again whenever `google.rpc.Code.NOT_FOUND`"]
        #[doc = " is returned from any controller method."]
        #[doc = ""]
        #[doc = " This protocol allows the controller service to disable debuggees, recover"]
        #[doc = " from data loss, or change the `debuggee_id` format. Agents must handle"]
        #[doc = " `debuggee_id` value changing upon re-registration."]
        pub async fn register_debuggee(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterDebuggeeRequest>,
        ) -> Result<tonic::Response<super::RegisterDebuggeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Controller2/RegisterDebuggee",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of all active breakpoints for the debuggee."]
        #[doc = ""]
        #[doc = " The breakpoint specification (`location`, `condition`, and `expressions`"]
        #[doc = " fields) is semantically immutable, although the field values may"]
        #[doc = " change. For example, an agent may update the location line number"]
        #[doc = " to reflect the actual line where the breakpoint was set, but this"]
        #[doc = " doesn't change the breakpoint semantics."]
        #[doc = ""]
        #[doc = " This means that an agent does not need to check if a breakpoint has changed"]
        #[doc = " when it encounters the same breakpoint on a successive call."]
        #[doc = " Moreover, an agent should remember the breakpoints that are completed"]
        #[doc = " until the controller removes them from the active list to avoid"]
        #[doc = " setting those breakpoints again."]
        pub async fn list_active_breakpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListActiveBreakpointsRequest>,
        ) -> Result<tonic::Response<super::ListActiveBreakpointsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Controller2/ListActiveBreakpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the breakpoint state or mutable fields."]
        #[doc = " The entire Breakpoint message must be sent back to the controller service."]
        #[doc = ""]
        #[doc = " Updates to active breakpoint fields are only allowed if the new value"]
        #[doc = " does not change the breakpoint specification. Updates to the `location`,"]
        #[doc = " `condition` and `expressions` fields should not alter the breakpoint"]
        #[doc = " semantics. These may only make changes such as canonicalizing a value"]
        #[doc = " or snapping the location to the correct line of code."]
        pub async fn update_active_breakpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateActiveBreakpointRequest>,
        ) -> Result<tonic::Response<super::UpdateActiveBreakpointResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Controller2/UpdateActiveBreakpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for Controller2Client<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for Controller2Client<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Controller2Client {{ ... }}")
        }
    }
}
/// Request to set a breakpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBreakpointRequest {
    /// Required. ID of the debuggee where the breakpoint is to be set.
    #[prost(string, tag = "1")]
    pub debuggee_id: std::string::String,
    /// Required. Breakpoint specification to set.
    /// The field `location` of the breakpoint must be set.
    #[prost(message, optional, tag = "2")]
    pub breakpoint: ::std::option::Option<Breakpoint>,
    /// Required. The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    #[prost(string, tag = "4")]
    pub client_version: std::string::String,
}
/// Response for setting a breakpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBreakpointResponse {
    /// Breakpoint resource.
    /// The field `id` is guaranteed to be set (in addition to the echoed fileds).
    #[prost(message, optional, tag = "1")]
    pub breakpoint: ::std::option::Option<Breakpoint>,
}
/// Request to get breakpoint information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBreakpointRequest {
    /// Required. ID of the debuggee whose breakpoint to get.
    #[prost(string, tag = "1")]
    pub debuggee_id: std::string::String,
    /// Required. ID of the breakpoint to get.
    #[prost(string, tag = "2")]
    pub breakpoint_id: std::string::String,
    /// Required. The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    #[prost(string, tag = "4")]
    pub client_version: std::string::String,
}
/// Response for getting breakpoint information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBreakpointResponse {
    /// Complete breakpoint state.
    /// The fields `id` and `location` are guaranteed to be set.
    #[prost(message, optional, tag = "1")]
    pub breakpoint: ::std::option::Option<Breakpoint>,
}
/// Request to delete a breakpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBreakpointRequest {
    /// Required. ID of the debuggee whose breakpoint to delete.
    #[prost(string, tag = "1")]
    pub debuggee_id: std::string::String,
    /// Required. ID of the breakpoint to delete.
    #[prost(string, tag = "2")]
    pub breakpoint_id: std::string::String,
    /// Required. The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    #[prost(string, tag = "3")]
    pub client_version: std::string::String,
}
/// Request to list breakpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBreakpointsRequest {
    /// Required. ID of the debuggee whose breakpoints to list.
    #[prost(string, tag = "1")]
    pub debuggee_id: std::string::String,
    /// When set to `true`, the response includes the list of breakpoints set by
    /// any user. Otherwise, it includes only breakpoints set by the caller.
    #[prost(bool, tag = "2")]
    pub include_all_users: bool,
    /// When set to `true`, the response includes active and inactive
    /// breakpoints. Otherwise, it includes only active breakpoints.
    #[prost(bool, tag = "3")]
    pub include_inactive: bool,
    /// When set, the response includes only breakpoints with the specified action.
    #[prost(message, optional, tag = "4")]
    pub action: ::std::option::Option<list_breakpoints_request::BreakpointActionValue>,
    /// This field is deprecated. The following fields are always stripped out of
    /// the result: `stack_frames`, `evaluated_expressions` and `variable_table`.
    #[prost(bool, tag = "5")]
    pub strip_results: bool,
    /// A wait token that, if specified, blocks the call until the breakpoints
    /// list has changed, or a server selected timeout has expired.  The value
    /// should be set from the last response. The error code
    /// `google.rpc.Code.ABORTED` (RPC) is returned on wait timeout, which
    /// should be called again with the same `wait_token`.
    #[prost(string, tag = "6")]
    pub wait_token: std::string::String,
    /// Required. The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    #[prost(string, tag = "8")]
    pub client_version: std::string::String,
}
pub mod list_breakpoints_request {
    /// Wrapper message for `Breakpoint.Action`. Defines a filter on the action
    /// field of breakpoints.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BreakpointActionValue {
        /// Only breakpoints with the specified action will pass the filter.
        #[prost(enumeration = "super::breakpoint::Action", tag = "1")]
        pub value: i32,
    }
}
/// Response for listing breakpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBreakpointsResponse {
    /// List of breakpoints matching the request.
    /// The fields `id` and `location` are guaranteed to be set on each breakpoint.
    /// The fields: `stack_frames`, `evaluated_expressions` and `variable_table`
    /// are cleared on each breakpoint regardless of its status.
    #[prost(message, repeated, tag = "1")]
    pub breakpoints: ::std::vec::Vec<Breakpoint>,
    /// A wait token that can be used in the next call to `list` (REST) or
    /// `ListBreakpoints` (RPC) to block until the list of breakpoints has changes.
    #[prost(string, tag = "2")]
    pub next_wait_token: std::string::String,
}
/// Request to list debuggees.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDebuggeesRequest {
    /// Required. Project number of a Google Cloud project whose debuggees to list.
    #[prost(string, tag = "2")]
    pub project: std::string::String,
    /// When set to `true`, the result includes all debuggees. Otherwise, the
    /// result includes only debuggees that are active.
    #[prost(bool, tag = "3")]
    pub include_inactive: bool,
    /// Required. The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    #[prost(string, tag = "4")]
    pub client_version: std::string::String,
}
/// Response for listing debuggees.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDebuggeesResponse {
    /// List of debuggees accessible to the calling user.
    /// The fields `debuggee.id` and `description` are guaranteed to be set.
    /// The `description` field is a human readable field provided by agents and
    /// can be displayed to users.
    #[prost(message, repeated, tag = "1")]
    pub debuggees: ::std::vec::Vec<Debuggee>,
}
#[doc = r" Generated client implementations."]
pub mod debugger2_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Debugger service provides the API that allows users to collect run-time"]
    #[doc = " information from a running application, without stopping or slowing it down"]
    #[doc = " and without modifying its state.  An application may include one or"]
    #[doc = " more replicated processes performing the same work."]
    #[doc = ""]
    #[doc = " A debugged application is represented using the Debuggee concept. The"]
    #[doc = " Debugger service provides a way to query for available debuggees, but does"]
    #[doc = " not provide a way to create one.  A debuggee is created using the Controller"]
    #[doc = " service, usually by running a debugger agent with the application."]
    #[doc = ""]
    #[doc = " The Debugger service enables the client to set one or more Breakpoints on a"]
    #[doc = " Debuggee and collect the results of the set Breakpoints."]
    pub struct Debugger2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> Debugger2Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Sets the breakpoint to the debuggee."]
        pub async fn set_breakpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::SetBreakpointRequest>,
        ) -> Result<tonic::Response<super::SetBreakpointResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Debugger2/SetBreakpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets breakpoint information."]
        pub async fn get_breakpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBreakpointRequest>,
        ) -> Result<tonic::Response<super::GetBreakpointResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Debugger2/GetBreakpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the breakpoint from the debuggee."]
        pub async fn delete_breakpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBreakpointRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Debugger2/DeleteBreakpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all breakpoints for the debuggee."]
        pub async fn list_breakpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBreakpointsRequest>,
        ) -> Result<tonic::Response<super::ListBreakpointsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Debugger2/ListBreakpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the debuggees that the user has access to."]
        pub async fn list_debuggees(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDebuggeesRequest>,
        ) -> Result<tonic::Response<super::ListDebuggeesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouddebugger.v2.Debugger2/ListDebuggees",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for Debugger2Client<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for Debugger2Client<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Debugger2Client {{ ... }}")
        }
    }
}
