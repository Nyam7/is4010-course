"""
Week06 lab module mirror for tests that import `lab06`.

This duplicates the top-level implementation so pytest collection from the
`week06/tests` directory can import the module as a local name.
"""

class Book:
    """A simple Book class with title, author, and year."""

    def __init__(self, title: str, author: str, year: int):
        self.title = title
        self.author = author
        self.year = year

    def __str__(self) -> str:
        return f'"{self.title}" by {self.author} ({self.year})'

    def get_age(self) -> int:
        current_year = 2025
        return current_year - self.year


class EBook(Book):
    """EBook extends Book by adding a file_size attribute (in MB)."""

    def __init__(self, title: str, author: str, year: int, file_size: int):
        super().__init__(title, author, year)
        self.file_size = file_size

    def __str__(self) -> str:
        parent_str = super().__str__()
        return f"{parent_str} ({self.file_size} MB)"


if __name__ == "__main__":
    b = Book("The Hobbit", "J.R.R. Tolkien", 1937)
    print(b)
    print("Age:", b.get_age())

    eb = EBook("Dune", "Frank Herbert", 1965, 5)
    print(eb)
    print("EBook age:", eb.get_age())
