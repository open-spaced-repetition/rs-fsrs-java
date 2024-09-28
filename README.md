# rs-fsrs-java

Java binding for rs-fsrs, WIP.

## Usage

see [example](./com/example/fsrs/FSRS.java)

## Develop

see [Makefile](./Makefile)

```
make java_run
```

## TODO

make api more friendly.

ideal style

```java
FSRS fsrs = new FSRS();
Card card = new Card();
ScheduledCards scheduled_cards = fsrs.schedule(card, Instant.now());
Card updatedCard = scheduled_cards.selectCard(Rating.Easy);
System.out.println(updatedCard.getLog().toString());
```
