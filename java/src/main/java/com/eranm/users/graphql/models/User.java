package com.eranm.users.graphql.models;

import lombok.Data;
import org.springframework.data.mongodb.core.mapping.FieldType;
import org.springframework.data.mongodb.core.mapping.MongoId;

import java.util.Date;
import java.util.List;

@Data
public class User {
    @MongoId(value = FieldType.OBJECT_ID)
    private String _id;
    private String firstName;
    private String lastName;
    private String email;
    private String phone;
    private Date updateDate;
    private Long ticket;
    private Date ticketExpiration;
    private Date registerDate;
    private List<Product> products;
}
