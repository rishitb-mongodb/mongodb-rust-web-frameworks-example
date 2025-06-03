# MongoDB Restaurants CRUD with Multiple Web Frameworks

This project demonstrates how to implement a CRUD application for a restaurant database using MongoDB and various Rust web frameworks.

## Prerequisites

- Rust (latest stable version)
- MongoDB running locally or accessible via network
- MongoDB Atlas [sample dataset](https://www.mongodb.com/docs/atlas/sample-data/) loaded in

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and configure your MongoDB connection string
3. Load the sample restaurants database if not already loaded:
```bash
mongosh
use sample_restaurants
```

## Running the Application

```bash
cargo run
```

The application will prompt you to choose a web framework implementation:

1. None (MongoDB driver only) - Interactive CLI
2. Actix Web (http://localhost:8080)
3. Axum (http://localhost:8081)
4. Rocket (http://localhost:8082)
5. Warp (http://localhost:8083)
6. Tide (http://localhost:8084)

## API Endpoints (for Web Framework Implementations)

All web framework implementations expose the same REST API endpoints:

### Create Restaurant
- POST `/api/restaurants`
- Body: Restaurant JSON

### List Restaurants
- GET `/api/restaurants`
- Returns first 10 restaurants

### Get Restaurant
- GET `/api/restaurants/{id}`
- Returns restaurant by ObjectId

### Update Restaurant
- PUT `/api/restaurants/{id}`
- Body: Update JSON document

### Delete Restaurant
- DELETE `/api/restaurants/{id}`
- Deletes restaurant by ObjectId

## Sample Restaurant Document

```json
{
  "address": {
    "building": "8825",
    "coord": [-73.8803827, 40.7643124],
    "street": "Astoria Boulevard",
    "zipcode": "11369"
  },
  "borough": "Queens",
  "cuisine": "American",
  "grades": [
    {
      "date": "2014-11-15T00:00:00.000Z",
      "grade": "Z",
      "score": 38
    }
  ],
  "name": "Brunos On The Boulevard",
  "restaurant_id": "40356151"
}
```

## Implementation Details

- `src/models/restaurant.rs` - Restaurant data model
- `src/db/mongodb.rs` - MongoDB repository implementation
- `src/frameworks/` - Web framework implementations
- `src/error.rs` - Error handling
- `src/main.rs` - Framework selection and startup

## Testing the API

You can test the API endpoints using curl, Postman, or any HTTP client. Example curl commands:

```bash
# List restaurants
curl http://localhost:8080/api/restaurants

# Get restaurant by ID
curl http://localhost:8080/api/restaurants/5eb3d668b31de5d588f42930

# Create restaurant
curl -X POST http://localhost:8080/api/restaurants \
  -H "Content-Type: application/json" \
  -d '{"name":"New Restaurant","borough":"Manhattan",...}'

# Update restaurant
curl -X PUT http://localhost:8080/api/restaurants/5eb3d668b31de5d588f42930 \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Name"}'

# Delete restaurant
curl -X DELETE http://localhost:8080/api/restaurants/5eb3d668b31de5d588f42930
```

Replace the port number (8080) with the appropriate port for your chosen framework:
- Actix Web: 8080
- Axum: 8081
- Rocket: 8082
- Warp: 8083
- Tide: 8084