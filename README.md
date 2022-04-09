# Gurumemo

Using [Yelp API](https://www.yelp.com/developers/documentation/v3).

You shoud get API KEY.

## Environment

### Volta: Node.js version manager

<https://volta.sh/>

### Docker, Docker Compose

- <https://docs.docker.jp/engine/installation/index.html>
- <https://docs.docker.jp/compose/install.html>

## Frontend

Next.js

- <https://nextjs.org/>

```bash
cd client
npm run dev
```

## Backend

axum

- <https://docs.rs/axum/latest/axum/>

```bash
cd server
cargo build

# Start web server
cargo run --bin server

# input Yelp data to MongoDB
# You can change the parameters. Detail: src/bin/get_yelp_data_to_mongo.rs
cargo run --bin get_yelp_data_to_mongo -- --latitude 35.69059985184279 --longitude 139.70279058434141 --radius=300
```

## Database

You can use docker-compose to start mongodb and mongo-express.

```sh
docker-compose -f docker/docker-compose.yml up -d
```

-　MongoDB

<https://www.mongodb.com/>

in MongoDB docker container

```bash
# connection to db
mongo -u root -p

# use yelp db
use yelp

# Search all
db.business.find()

# count
db.business.find().count

# delete duplicated contents
# reference https://qiita.com/sey323/items/700a47bf5f12e04fd4d0
db.business.aggregate([
  { $group:
    {
    _id: "$id",
    dups: { "$addToSet": "$_id" },
    count: { "$sum": 1 }
    }
  },
  { $match:
    {
    count: { "$gt": 1 }
    }
  }
]).forEach(function(doc) {
    doc.dups.shift();
    db.business.remove({_id : {$in: doc.dups }});
})
```

- mongo-express

http://localhost:8081
