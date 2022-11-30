-- Place your queries here. Docs available https://www.hugsql.org/
-- :name create-user! :! :n
-- :doc Create a user
insert into users (email, pass)
values (:email, :pass)

-- :name check-password :? :1
-- :doc Check password; if password works, return the user
select * from users
where email = :email
and pass = digest(:pass, 'sha256')
