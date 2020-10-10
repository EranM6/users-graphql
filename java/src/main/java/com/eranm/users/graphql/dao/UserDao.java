package com.eranm.users.graphql.dao;

import com.eranm.users.graphql.models.User;
import com.eranm.users.mongoDb.MongoDb;
import org.springframework.context.annotation.Configuration;

@Configuration
public class UserDao {
    private final MongoDb mongoDb;

    public UserDao(MongoDb mongoDb) {
        this.mongoDb = mongoDb;
    }

    public User getUserById(String id) {
        return mongoDb.getUser(id);
    }

    public User getUserByField(String field, Object value) {
        return mongoDb.getUser(field, value);
    }
}
