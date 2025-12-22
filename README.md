# rust-grpc-server-demo

## protovalidateのインストール

```bash
cargo make install-protovalidate
```

## protobufのコード生成

```bash
cargo build
```

## サーバーの起動

```bash
cargo run
```

## Example

```bash
grpcurl -plaintext -d '{"name": "user1", "email": "test@test.com"}' localhost:50051 user.v1.UserService.CreateUser
{
  "id": "a53e674a-8339-4d7d-9abe-e83ca743245e"
}
```

```bash
grpcurl -plaintext -d '{"id": "a53e674a-8339-4d7d-9abe-e83ca743245e"}' localhost:50051 user.v1.UserService.GetUser
{
  "id": "a53e674a-8339-4d7d-9abe-e83ca743245e",
  "name": "user1",
  "email": "test@test.com"
}
```

```bash
grpcurl -plaintext -d '{"city": "tokyo"}' localhost:50051 weather.v1.WeatherService.GetWeather
{
  "weather": "WEATHER_SUNNY"
}
```

```bash
grpcurl -plaintext -d '{"city": "okinawa"}' localhost:50051 weather.v1.WeatherService.GetWeather

ERROR:
  Code: InvalidArgument
  Message: must be one of these values: [ 'tokyo', 'osaka' ]
  Details:
  1)    {
          "@type": "type.googleapis.com/buf.validate.Violations",
          "violations": [
            {
              "ruleId": "string.in",
              "message": "must be one of these values: [ 'tokyo', 'osaka' ]",
              "field": {
                "elements": [
                  {
                    "fieldNumber": 1,
                    "fieldName": "city",
                    "fieldType": "TYPE_STRING"
                  }
                ]
              },
              "rule": {
                "elements": [
                  {
                    "fieldNumber": 14,
                    "fieldName": "string",
                    "fieldType": "TYPE_MESSAGE"
                  },
                  {
                    "fieldNumber": 10,
                    "fieldName": "in",
                    "fieldType": "TYPE_STRING"
                  }
                ]
              }
            }
          ]
        }
```

