use serde::{Deserialize, Serialize};

//used for testing
#[allow(dead_code)]
const SERVICE_1: &str = r#"{
  "service": "service_1",
  "description": "a test service",
  "actions": [
    {
      "name": "action #1",
      "description": "action #1 does something",
      "parameters": [
        {
          "name": "a number #1",
          "description": "this number can be only positive and is required!",
          "type": "Uint32",
          "required": true,
          "default": null
        },
        {
          "name": "a number #1",
          "description": "this number can be positive and negative and is not required",
          "type": "Int32",
          "required": false,
          "default": "0"
        }
      ],
      "outputs": [
        {
          "name": "message",
          "description": "a message of success or failure",
          "type": {
            "Enum": [
              "ENUM_1",
              "ENUM_2"
            ]
          }
        }
      ]
    }
  ]
}"#;

/// paramters types of actions - serilizable as strings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ParameterType {
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    String,
    Enum(Vec<String>),
}

/// outputs of a possible action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: ParameterType,
}

/// Parameters of a possible action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: ParameterType,
    pub required: bool,
    pub default: Option<String>,
}

/// A service is a collection of actions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub outputs: Vec<Output>,
}

///Structure of a service API description which is serialized to JSON
/// Contains name, description and actions
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServiceMeta {
    pub service: String,
    pub description: String,
    pub actions: Vec<Action>,
}

impl ServiceMeta {
    /// Creates a new service from a JSON string
    pub fn mock() -> ServiceMeta {
        serde_json::from_str(SERVICE_1).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

//---------------- COMMAND -------------------

///command parameters to be sent as a command to a service. Inner member of the Command stuct
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestParameter {
    key: String,
    value: String,
    #[serde(rename = "type")]
    type_: ParameterType,
}

/// A command sent to the service as a json object of the form:
/// /service/action POST {
///  "parameter_1": "value_1",
/// "parameter_2": "value_2"
/// }
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServiceRequest {
    pub action: String,
    pub parameters: Vec<RequestParameter>,
}

impl ServiceRequest {
    /// create a new command
    pub fn new(action: String) -> ServiceRequest {
        ServiceRequest {
            action,
            parameters: Vec::new(),
        }
    }

    /// add a parameter to the command
    pub fn add_parameter(&mut self, key: String, value: String, type_: ParameterType) {
        self.parameters.push(RequestParameter { key, value, type_ });
    }
}

//---------------- RESPONSE -------------------

/// Response from a service as a json object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServiceResponse {
    message: String,
    pub parameters: Vec<RequestParameter>,
}

impl ServiceResponse {
    /// create a new response
    pub fn new(message: String) -> ServiceResponse {
        ServiceResponse {
            message,
            parameters: Vec::new(),
        }
    }

    /// add a parameter to the response
    pub fn add_parameter(&mut self, key: String, value: String, type_: ParameterType) {
        self.parameters.push(RequestParameter { key, value, type_ });
    }
}

//---------------- TESTING -------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_service() -> ServiceMeta {
        ServiceMeta {
            service: "service_1".to_string(),
            description: "a test service".to_string(),
            actions: vec![Action {
                name: "action #1".to_string(),
                description: "action #1 does something".to_string(),
                parameters: vec![
                    Parameter {
                        name: "a number #1".to_string(),
                        description: "this number can be only positive and is required!"
                            .to_string(),
                        type_: ParameterType::Uint32,
                        required: true,
                        default: None,
                    },
                    Parameter {
                        name: "a number #1".to_string(),
                        description: "this number can be positive and negative and is not required"
                            .to_string(),
                        type_: ParameterType::Int32,
                        required: false,
                        default: Some("0".to_string()),
                    },
                ],
                outputs: vec![Output {
                    name: "message".to_string(),
                    description: "a message of success or failure".to_string(),
                    type_: ParameterType::Enum(vec!["ENUM_1".to_string(), "ENUM_2".to_string()]),
                }],
            }],
        }
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn serialize_json() {
        let service = mock_service();
        let json = serde_json::to_string_pretty(&service).unwrap();
        println!("{}", json);
        assert_eq!(json, SERVICE_1.to_string());
    }

    #[test]
    fn deserialize_json() {
        let service = mock_service();
        let desirialized = serde_json::from_str(&SERVICE_1).unwrap();
        assert_eq!(service, desirialized);
    }
}