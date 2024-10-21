# Type hints in python.

# Firstly why type hints?
# - Type hints are a form of self-documentation.
# - Type hints can aid development as IDEs provide better autocompletion
#    and type checking.
# - Type hints can help catch bugs though static analysis.

# Why not type hints?
# - Type hints can be hard to get right and sync time for generic code.
# - Type hints may slow you down for through away code.


# Example: Shopping cart

import logging
from collections.abc import Callable  # type: ignore
from enum import Enum
from typing import Iterable, Protocol, SupportsIndex, Tuple, Union, overload

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


class Customer:
    def __init__(self, name: str, email: str, balance: float = 0) -> None:
        self.name = name
        self.email = email
        self.balance = balance


class Category(Enum):
    ELECTRONICS = "electronics"
    CLOTHING = "clothing"
    GROCERIES = "groceries"


class Product:
    def __init__(self, name: str, price: float, category: Category) -> None:
        self.name = name
        self.price = price
        self.category = category


class ShoppingCart:
    products: list[Product]

    def __init__(self) -> None:
        self.products = []

    def __iter__(self) -> Iterable[Product]:
        return iter(self.products)

    def __len__(self) -> int:
        return len(self.products)

    @overload
    def __getitem__(self, i: SupportsIndex, /) -> Product: ...
    @overload
    def __getitem__(self, s: slice, /) -> list[Product]: ...
    def __getitem__(
        self, index: Union[SupportsIndex, slice]
    ) -> Union[Product, list[Product]]:
        return self.products[index]

    def add_product(self, product: Product) -> None:
        self.products.append(product)

    def total(self) -> float:
        return sum(product.price for product in self.products)

    def __str__(self) -> str:
        items = "\n".join(
            f"  - {product.name} - £{product.price}" for product in self.products
        )
        return f"ShoppingCart\n{items}"


class DiscountStrategy(Protocol):
    def __call__(self, cart: ShoppingCart) -> None | Tuple[str, float]:
        """Return a tuple of discount description and discount amount."""
        pass


# Could also be written as:
# DiscountStrategy = Callable[[ShoppingCart], None | Tuple[str, float]]


class Order:
    def __init__(self, customer: Customer, cart: ShoppingCart) -> None:
        self.customer = customer
        self.cart = cart


def process_checkout(
    customer: Customer,
    cart: ShoppingCart,
    discount_strategies: list[DiscountStrategy] = [],
) -> Order | None:
    discount_cart_total = cart.total()
    logger.info(f"Price before discount: £{cart.total()}")
    for strategy in discount_strategies:
        discount = strategy(cart)
        if discount is not None:
            description, amount = discount
            logger.info(f"Applying discount: {description} - £{amount:.2f}")
            discount_cart_total = max(0, discount_cart_total - amount)
    logger.info(f"Price after discount: £{discount_cart_total}")
    if customer.balance < cart.total():
        msg = f"Insufficient balance {customer.balance} < {cart.total()}"
        logger.info(msg)
        raise ValueError(msg)
    customer.balance -= discount_cart_total
    logger.info(f"Order created for {customer.name}")
    return Order(customer, cart)


def example_alice() -> None:
    customer = Customer("Alice", "alice.dj@gmail.com", 1000)
    alice_cart = ShoppingCart()

    laptop = Product("Laptop", 600, Category.ELECTRONICS)
    t_shirt = Product("T-shirt", 20, Category.CLOTHING)
    milk = Product("Milk", 2, Category.GROCERIES)
    orange_juice = Product("Orange Juice", 3, Category.GROCERIES)

    alice_cart.add_product(laptop)
    alice_cart.add_product(t_shirt)
    alice_cart.add_product(milk)
    alice_cart.add_product(orange_juice)

    logger.info(alice_cart)

    process_checkout(customer, alice_cart)


def ten_percent_discount(cart: ShoppingCart) -> Tuple[str, float]:
    discount = cart.total() * 0.1
    return ("10% discount", discount)


class ByOneGetOneFree:
    def __init__(self, product: Product) -> None:
        self.product = product

    def __call__(self, cart: ShoppingCart) -> None | Tuple[str, float]:
        """return free item count * base price"""
        n_products = sum(1 for product in cart.products if product == self.product)
        discount_value = (n_products // 2) * self.product.price
        if discount_value > 0:
            return (f"Buy one get one free {self.product.name}", discount_value)
        return None


def example_bob() -> None:

    apple = Product("Apple", 1, Category.GROCERIES)
    orange = Product("Orange", 2, Category.GROCERIES)

    customer = Customer("Bob", "bob.builder@gmail.com", 1000)
    bob_cart = ShoppingCart()

    by_one_apple_get_one_free = ByOneGetOneFree(apple)

    discounts: list[DiscountStrategy] = [
        by_one_apple_get_one_free,
        ten_percent_discount,
    ]

    bob_cart.add_product(apple)
    bob_cart.add_product(apple)
    bob_cart.add_product(apple)
    bob_cart.add_product(apple)
    bob_cart.add_product(orange)

    logger.info(bob_cart)
    process_checkout(customer, bob_cart, discounts)


if __name__ == "__main__":
    example_alice()
    example_bob()
