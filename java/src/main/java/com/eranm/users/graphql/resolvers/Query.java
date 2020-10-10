package com.eranm.users.graphql.resolvers;

import com.coxautodev.graphql.tools.GraphQLQueryResolver;
import com.eranm.users.graphql.dao.UserDao;
import com.eranm.users.graphql.models.User;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;
import org.springframework.stereotype.Component;

@Slf4j
@Configuration
@Component
public class Query implements GraphQLQueryResolver {
    private final UserDao userDao;

    @Autowired
    public Query(UserDao userDao) {
        this.userDao = userDao;
    }

    public User getUserById(String id) {
        return userDao.getUserById(id);
    }

    public User getUserByMail(String email) {
        return userDao.getUserByField("email", email);
    }
}