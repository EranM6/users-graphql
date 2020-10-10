package com.eranm.users.graphql.resolvers;

import com.coxautodev.graphql.tools.GraphQLResolver;
import com.eranm.users.enums.Status;
import com.eranm.users.graphql.models.Product;
import org.springframework.stereotype.Component;

@Component
public class ProductResolver implements GraphQLResolver<Product> {
    public Status getStatus(Product product) {
        return Status.getStatus(product.getStatusCode());
    }
}
