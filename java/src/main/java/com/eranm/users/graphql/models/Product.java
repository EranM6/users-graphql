package com.eranm.users.graphql.models;

import com.eranm.users.enums.Status;
import lombok.Data;

@Data
public class Product {
    private int prodNum;
    private int statusCode;
    private int saleCode;
    private Status status;
}
