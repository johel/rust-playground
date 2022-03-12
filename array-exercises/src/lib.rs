mod my_number;
pub use my_number::LIST;

mod users;
pub use users::{Sex, User, UserProduct, USERS, USER_PRODUCTS};

#[cfg(test)]
mod tests {
    use crate::LIST;
    use crate::{USERS, USER_PRODUCTS};

    #[test]
    fn it_works() {
        let result = 2;
        assert_eq!(result, LIST.len());
    }

    #[test]
    fn has_users() {
        let result = true;
        assert_eq!(result, USERS.len() > 0);
    }

    #[test]
    fn has_user_products() {
        let result = true;
        assert_eq!(result, USER_PRODUCTS.len() > 0);
    }
}
