package com.eranm.users.graphql.resolvers;

import com.coxautodev.graphql.tools.GraphQLResolver;
import com.eranm.users.graphql.models.User;
import org.springframework.stereotype.Component;

import java.text.SimpleDateFormat;

@Component
public class UserResolver implements GraphQLResolver<User> {
    public String getUpdateDate(User user) {
//        SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss");
        SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd");
        return formatter.format(user.getUpdateDate());
    }

    public String getTicketExpiration(User user) {
//        SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss");
        SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd");
        return formatter.format(user.getTicketExpiration());
    }

    public String getRegisterDate(User user) {
//        SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss");
        SimpleDateFormat formatter = new SimpleDateFormat("yyyy-MM-dd");
        return formatter.format(user.getRegisterDate());
    }

    public Boolean getIsPayingUser(User user) {
        return !(user.getProducts() == null || user.getProducts().isEmpty());
    }
}
