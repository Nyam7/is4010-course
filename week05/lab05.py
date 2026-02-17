"""
Lab 05 - Functions and Error Handling
Refactored user processing script with robust error handling.
"""

# Original users list
users = [
    {"name": "alice", "age": 30, "is_active": True, "email": "alice@example.com"},
    {"name": "bob", "age": 25, "is_active": False},
    {"name": "charlie", "age": 35, "is_active": True, "email": "charlie@example.com"},
    {"name": "david", "age": "unknown", "is_active": False},
]

def calculate_average_age(user_list):
    """
    Calculate the average age of users.

    Parameters
    ----------
    user_list : list of dict
        A list of user dictionaries containing an 'age' key.

    Returns
    -------
    float
        The average age of users with valid integer ages.
        Returns 0.0 if no valid ages exist or list is empty.
    """
    try:
        if not user_list:
            raise ZeroDivisionError("Empty user list")

        total_age = 0
        count = 0

        for user in user_list:
            age = user.get("age")
            if isinstance(age, int):
                total_age += age
                count += 1

        if count == 0:
            raise ZeroDivisionError("No valid age data")

        return total_age / count

    except ZeroDivisionError:
        print("Error: cannot calculate average age of an empty or invalid list.")
        return 0.0


def get_active_user_emails(user_list):
    """
    Get a list of emails for active users.

    Parameters
    ----------
    user_list : list of dict
        A list of user dictionaries.

    Returns
    -------
    list
        A list of email addresses for users where
        'is_active' is True and 'email' exists.
    """
    try:
        if not user_list:
            return []

        active_emails = []

        for user in user_list:
            try:
                if user.get("is_active") and user.get("email"):
                    active_emails.append(user["email"])
            except KeyError:
                continue

        return active_emails

    except TypeError:
        print("Error: invalid user list provided.")
        return []


if __name__ == "__main__":
    avg_age = calculate_average_age(users)
    print(f"average user age: {avg_age:.2f}")

    active_emails = get_active_user_emails(users)
    print(f"active user emails: {active_emails}")
