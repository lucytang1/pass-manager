masterpassword
EK = First.H(Algo(SALT + PASS))
AK = Second.H(Algo(SALT + PASS))


Schema:
    
APIs:
/register
Request:
{
    email,
    user_key,
}

Response:
{
    email,
    vault,
    salt
}

/auth
Request:
{
    email,
    user_key
}

Response:
{
    email,
    vault,
    salt
}