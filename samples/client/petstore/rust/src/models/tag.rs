/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

// Tag : A tag for a pet

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
  #[serde(rename = "id")] id: Option<i64>,
  #[serde(rename = "name")] name: Option<String>
}

impl Tag {
  // A tag for a pet
  pub fn new() -> Tag {
    Tag {
      id: None,
      name: None
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Tag {
    self.id = Some(id);
    self
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Tag {
    self.name = Some(name);
    self
  }

}



