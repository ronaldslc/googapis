initSidebarItems({"enum":[["VariableState","The `VariableState` describes the last known state of the variable and is used during a `variables().watch` call to distinguish the state of the variable."]],"mod":[["end_condition","Nested message and enum types in `EndCondition`."],["runtime_config_manager_client","Generated client implementations."],["variable","Nested message and enum types in `Variable`."]],"struct":[["CreateConfigRequest","Creates a RuntimeConfig resource."],["CreateVariableRequest","Request for the `CreateVariable()` method."],["CreateWaiterRequest","Request message for `CreateWaiter()` method."],["DeleteConfigRequest","Request for the `DeleteConfig()` method."],["DeleteVariableRequest","Request for the `DeleteVariable()` method."],["DeleteWaiterRequest","Request for the `DeleteWaiter()` method."],["EndCondition","The condition that a Waiter resource is waiting for."],["GetConfigRequest","Gets a RuntimeConfig resource."],["GetVariableRequest","Request for the `GetVariable()` method."],["GetWaiterRequest","Request for the `GetWaiter()` method."],["ListConfigsRequest","Request for the `ListConfigs()` method."],["ListConfigsResponse","`ListConfigs()` returns the following response. The order of returned objects is arbitrary; that is, it is not ordered in any particular way."],["ListVariablesRequest","Request for the `ListVariables()` method."],["ListVariablesResponse","Response for the `ListVariables()` method."],["ListWaitersRequest","Request for the `ListWaiters()` method."],["ListWaitersResponse","Response for the `ListWaiters()` method. Order of returned waiter objects is arbitrary."],["RuntimeConfig","A RuntimeConfig resource is the primary resource in the Cloud RuntimeConfig service. A RuntimeConfig resource consists of metadata and a hierarchy of variables."],["UpdateConfigRequest","Request message for `UpdateConfig()` method."],["UpdateVariableRequest","Request for the `UpdateVariable()` method."],["Variable","Describes a single variable within a RuntimeConfig resource. The name denotes the hierarchical variable name. For example, `ports/serving_port` is a valid variable name. The variable value is an opaque string and only leaf variables can have values (that is, variables that do not have any child variables)."],["Waiter","A Waiter resource waits for some end condition within a RuntimeConfig resource to be met before it returns. For example, assume you have a distributed system where each node writes to a Variable resource indidicating the node’s readiness as part of the startup process."],["WatchVariableRequest","Request for the `WatchVariable()` method."]]});