create database conference_registration;
create user conference_user with encrypted password 'conference_password';
grant all privileges on database conference_registration to conference_user;
\c conference_registration;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO conference_user;


