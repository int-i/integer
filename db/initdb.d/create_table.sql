CREATE TABLE members (                                       
    id     integer                 PRIMARY KEY,              
    name   character varying(8)    NOT NULL,                 
    phone  character varying(16),                            
    email  character varying(128),                           
    note   text                                              
);

