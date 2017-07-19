/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration, models};

pub struct StoreApiImpl<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> StoreApiImpl<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> StoreApiImpl<C> {
        StoreApiImpl {
            configuration: configuration,
        }
    }
}

pub trait StoreApi {
    fn DeleteOrder(&self, order_id: String) -> Box<Future<Item = (), Error = Error>>;
    fn GetInventory(&self, ) -> Box<Future<Item = ::std::collections::HashMap<String, i32>, Error = Error>>;
    fn GetOrderById(&self, order_id: i64) -> Box<Future<Item = Order, Error = Error>>;
    fn PlaceOrder(&self, body: Order) -> Box<Future<Item = Order, Error = Error>>;
}


impl<C: hyper::client::Connect>StoreApi for StoreApiImpl<C> {
    fn DeleteOrder(&self, order_id: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Delete,
            format!("{}/store/order/{orderId}", configuration.base_path, order_id=order_id));

    }

    fn GetInventory(&self, ) -> Box<Future<Item = super::::std::collections::HashMap<String, i32>, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("{}/store/inventory", configuration.base_path));

    }

    fn GetOrderById(&self, order_id: i64) -> Box<Future<Item = super::Order, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("{}/store/order/{orderId}", configuration.base_path, order_id=order_id));

    }

    fn PlaceOrder(&self, body: Order) -> Box<Future<Item = super::Order, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("{}/store/order", configuration.base_path, body=body));

        // body params
        let serialized = serde_json::to_string(body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

}
