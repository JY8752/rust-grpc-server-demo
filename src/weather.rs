use rand::prelude::IndexedRandom;
use tonic::{Request, Response, Status};

use proto::v1::{
    GetWeatherRequest, GetWeatherResponse, weather_service_server::WeatherServiceServer,
};

use crate::validate;

mod proto {
    pub mod v1 {
        tonic::include_proto!("weather.v1");
    }
}

pub struct WeatherService;

impl WeatherService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WeatherService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl proto::v1::weather_service_server::WeatherService for WeatherService {
    async fn get_weather(
        &self,
        request: Request<GetWeatherRequest>,
    ) -> Result<Response<GetWeatherResponse>, Status> {
        validate::execute(request, |_| async {
            // ランダムな天気を生成
            let weathers = [
                proto::v1::get_weather_response::Weather::Sunny,
                proto::v1::get_weather_response::Weather::Cloudy,
                proto::v1::get_weather_response::Weather::Rainy,
                proto::v1::get_weather_response::Weather::Snowy,
            ];
            let weather = weathers.choose(&mut rand::rng()).unwrap();

            Ok(Response::new(GetWeatherResponse {
                weather: (*weather).into(),
            }))
        })
        .await
    }
}

pub fn get_service() -> WeatherServiceServer<WeatherService> {
    WeatherServiceServer::new(WeatherService::new())
}
