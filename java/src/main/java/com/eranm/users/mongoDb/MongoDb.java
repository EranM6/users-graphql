package com.eranm.users.mongoDb;

import com.eranm.users.graphql.models.User;
import com.mongodb.ReadPreference;
import org.bson.types.ObjectId;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.data.mongodb.core.query.Criteria;
import org.springframework.data.mongodb.core.query.Query;

@Configuration
public class MongoDb {
    private final MongoTemplate mongoTemplate;

    @Autowired
    public MongoDb(MongoTemplate mongoTemplate) {
        this.mongoTemplate = mongoTemplate;
        this.mongoTemplate.setReadPreference(ReadPreference.secondaryPreferred());
    }

    public User getUser(String id) {
        return mongoTemplate.findById(new ObjectId(id), User.class, "users");
    }

    public User getUser(String field, Object value) {
        return mongoTemplate.findOne(
                new Query(Criteria.where(field).is(value)),
                User.class,
                "users"
        );
    }
}
