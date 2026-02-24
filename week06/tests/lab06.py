"""Local copy of lab06 module for tests to import directly from tests directory."""


class Book:
    def __init__(self, title: str, author: str, year: int):
        self.title = title
        self.author = author
        self.year = year

    def __str__(self) -> str:
        return f'"{self.title}" by {self.author} ({self.year})'

    def get_age(self) -> int:
        return 2025 - self.year


class EBook(Book):
    def __init__(self, title: str, author: str, year: int, file_size: int):
        super().__init__(title, author, year)
        self.file_size = file_size

    def __str__(self) -> str:
        return f"{super().__str__()} ({self.file_size} MB)"
