from setuptools import setup, find_packages

setup(
    name="virtualcan",
    description="Tools to create a virtual CAN bus.",
    version="0.0.1",
    author="Windel Bouwman",
    author_email="windel@windel.nl",
    packages=find_packages(),
    url="https://github.com/windelbouwman/virtualcan",
    classifiers=[
        "Development Status :: 1 - Planning",
        "License :: OSI Approved :: GNU General Public License v3 (GPLv3)",
        "Programming Language :: Python :: 3.7",
        "Topic :: Software Development :: Embedded Systems",
        "Topic :: Software Development :: Testing",
    ],
    entry_points={
        "can.interface": ["virtualcan = virtualcan.can:VirtualCanBus"],
    },
)
