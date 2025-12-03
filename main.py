import csv
import random

# Given data
Emotions = [
    "Happy",
    "Sad",
    "Angry",
    "Fear",
    "Disgust",
    "Surprise",
    "Neutral",
    "Excited",
    "Bored",
    "Confused",
    "Tired",
    "Anxious",
    "Proud",
    "Embarrassed",
    "Relaxed",
    "Frustrated",
    "Lonely",
    "Curious",
]

Categories = [
    "BasicNeeds",
    "Safety",
    "Feelings",
    "Commands",
    "Questions",
    "Pain",
    "Social",
    "Attention",
    "HelpRequest",
    "Urgent",
]

Tones = ["Soft", "Firm", "Urgent", "Playful", "Neutral", "Questioning", "Stressed"]

Symbols = [
    "🧒",  # child/kid
    "🙂",  # smile/happy
    "⚠️",  # warning/caution
    "📣",  # speak/shout
    "🍎",  # apple/food
    "✋",  # stop/hand
    "🏃",  # run/move
    "😢",  # sad/cry
    "❤️",  # love/heart
    "❗",  # important/exclamation
    "⭕",  # circle/ok
    # Expanded
    "👍",  # yes/approve
    "👎",  # no/disapprove
    "😃",  # excited/happy
    "😠",  # angry
    "😱",  # fear/scared
    "🤔",  # think/confused
    "😴",  # tired/sleep
    "🙌",  # celebrate/praise
    "🍔",  # food
    "💧",  # water/drink
    "🚫",  # no/not allowed
    "🤗",  # hug/welcome
    "🎉",  # party/celebrate
    "❓",  # question
    "🛑",  # stop/danger
    "🚶",  # walk
    # More added
    "🌞",  # sun/day
    "🌜",  # moon/night
    "💡",  # idea/light
    "📖",  # book/learn
    "🛌",  # bed/sleep
    "🧸",  # teddy/kid comfort
    "🎂",  # cake/birthday
    "📅",  # calendar/day/time
    "🚗",  # car/transport
    "🎵",  # music/sound
    "📱",  # phone/contact
    "⚽",  # sport/play
    "💊",  # medicine/health
    "🚿",  # shower/clean
    "🧼",  # soap/clean
    "🧑‍🏫",  # teacher/learn
    "🧑‍⚕️",  # doctor/health
    "🦷",  # tooth/dental
    "🍪",  # cookie/snack
    "🛒",  # shopping/cart
]

BasePhrases = [
    "Hello",
    "Hi",
    "Good morning",
    "Good night",
    "Thank you",
    "Please",
    "Sorry",
    "Goodbye",
    "I want {item}",
    "I need help with {item}",
    "I feel {emotion}",
    "Stop please",
    "More please",
    "I don’t like this",
    "I am hurt",
    "I am confused",
    "Where is {item}?",
    "Can you help me?",
    "I need a break",
    "I want to {action}",
    "Can I {action} please?",
    "I am {emotion} because I {action}",
]

Items = [
    "food",
    "drink",
    "toy",
    "tablet",
    "bathroom",
    "rest",
    "quiet",
    "music",
    "break",
    "jacket",
    "teacher",
    "friend",
    "water",
    "liquid",
    "snack",
    "medicine",
]

Actions = [
    "run",
    "walk",
    "sprint",
    "jog",
    "drink water",
    "eat food",
    "use bathroom",
    "rest",
    "play",
    "sleep",
    "talk",
    "listen",
    "help me",
    "stop",
    "wait",
]


def fill_phrase(template):
    phrase = template
    if "{item}" in phrase:
        phrase = phrase.replace("{item}", random.choice(Items))
    if "{emotion}" in phrase:
        phrase = phrase.replace("{emotion}", random.choice(Emotions))
    if "{action}" in phrase:
        phrase = phrase.replace("{action}", random.choice(Actions))
    return phrase


def calculate_safety_score(emotion, tone):
    score = 100
    if emotion in ["Angry", "Fear", "Disgust", "Frustrated", "Anxious", "Sad"]:
        score -= 30
    if tone in ["Urgent", "Stressed", "Firm"]:
        score -= 20
    return max(score, 0)


import time


def generate_phrases(n=100000):
    data = []
    seen = set()
    start_time = time.time()

    for i in range(n):
        base = random.choice(BasePhrases)
        phrase = fill_phrase(base)
        emotion = random.choice(Emotions)
        category = random.choice(Categories)
        tone = random.choice(Tones)
        symbol = random.choice(Symbols)
        safetyscore = calculate_safety_score(emotion, tone)

        row = (phrase, emotion, category, tone, symbol, safetyscore)
        if row not in seen:
            seen.add(row)
            data.append(row)

        # Progress & timing every 1000 items
        if (i + 1) % 1000 == 0:
            elapsed = time.time() - start_time
            rate = (i + 1) / elapsed
            remaining = (n - (i + 1)) / rate
            print(
                f"Generated {i + 1}/{n} phrases | "
                f"{rate:.2f} phrases/sec | "
                f"Elapsed: {elapsed:.1f}s | "
                f"ETA: {remaining:.1f}s"
            )

    return data


if __name__ == "__main__":
    phrases_data = generate_phrases(100000)

    total_rows = len(phrases_data)
    start_time = time.time()

    with open(
        "./phrases_dataset_expanded.csv", mode="w", newline="", encoding="utf-8"
    ) as f:
        writer = csv.writer(f)
        writer.writerow(
            ["phrase", "emotion", "category", "tone", "symbol", "safetyscore"]
        )

        for i, row in enumerate(phrases_data, 1):
            writer.writerow(row)

            if i % 1000 == 0 or i == total_rows:
                elapsed = time.time() - start_time
                rate = i / elapsed if elapsed > 0 else 0
                remaining = (total_rows - i) / rate if rate > 0 else 0
                print(
                    f"Writing CSV progress: {i}/{total_rows} rows | "
                    f"{rate:.2f} rows/sec | "
                    f"Elapsed: {elapsed:.1f}s | "
                    f"ETA: {remaining:.1f}s"
                )

    print("Dataset generated and saved to phrases_dataset_expanded.csv")
