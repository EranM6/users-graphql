package com.eranm.users;

import graphql.scalars.ExtendedScalars;
import graphql.schema.GraphQLScalarType;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class AppConfiguration {
    @Bean
    public GraphQLScalarType dateType() {
        return ExtendedScalars.Date;
    }
}
