import csv
import sys

import pyttsx3

CSV_FILE = "phrases_dataset_expanded.csv"
PAGE_SIZE = 8


def load_phrases(csv_file):
    phrases = []
    with open(csv_file, newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            phrases.append(row)
    return phrases


def unique_sorted_values(phrases, key):
    return sorted(set(p[key] for p in phrases))


def select_from_list(prompt, options, allow_skip=True):
    while True:
        print(f"\n{prompt}")
        for i, option in enumerate(options, 1):
            print(f"{i}. {option}")
        if allow_skip:
            print("0. Skip / No filter")

        choice = input("> ").strip()
        if choice == "0" and allow_skip:
            return None
        if choice.isdigit():
            idx = int(choice)
            if 1 <= idx <= len(options):
                return options[idx - 1]
        print("Invalid choice. Please try again.")


def filter_phrases(phrases, category=None, emotion=None, keyword=None):
    filtered = []
    for p in phrases:
        if category and p["category"] != category:
            continue
        if emotion and p["emotion"] != emotion:
            continue
        if keyword and keyword.lower() not in p["phrase"].lower():
            continue
        filtered.append(p)
    return filtered


def paginate_list(lst, page, page_size):
    start = page * page_size
    end = start + page_size
    return lst[start:end], len(lst)


def show_phrases(phrases, page):
    page_phrases, total = paginate_list(phrases, page, PAGE_SIZE)
    print(f"\nPhrases (page {page + 1}):")
    for i, p in enumerate(page_phrases, 1):
        print(f"{i}. {p['phrase']} {p['symbol']}")
    print(
        "\nCommands: number to speak | n-next page | p-previous page | b-back to filters | exit"
    )


def speak(text):
    # macOS: use the native NSSpeechSynthesizer driver ("nsss").
    # window use sapi5
    # linux use espeak
    engine = pyttsx3.init(driverName="nsss")
    try:
        # Optional: pick a reliable voice (e.g., Alex) if available.
        # for v in engine.getProperty("voices"):
        #     if v.id == "com.apple.speech.synthesis.voice.alex":
        #         engine.setProperty("voice", v.id)
        #         break

        engine.say(text)
        engine.runAndWait()
    finally:
        engine.stop()
        # Allow the engine to be GC'd between utterances.


def main():
    phrases = load_phrases(CSV_FILE)
    if not phrases:
        print("No phrases loaded.")
        sys.exit(1)

    while True:
        print("\n=== AAC App - Select Filters ===")

        categories = unique_sorted_values(phrases, "category")
        emotions = unique_sorted_values(phrases, "emotion")

        category = select_from_list("Select a Category:", categories)
        emotion = select_from_list("Select an Emotion:", emotions)

        common_keywords = sorted(
            set(
                word.lower()
                for phrase in phrases
                for word in phrase["phrase"].split()
                if len(word) > 2
            )
        )
        keyword = select_from_list(
            "Select a Keyword to filter phrases (or skip):", common_keywords
        )

        filtered_phrases = filter_phrases(phrases, category, emotion, keyword)

        if not filtered_phrases:
            print("\nNo phrases found with those filters. Try again.")
            continue

        page = 0
        while True:
            show_phrases(filtered_phrases, page)
            cmd = input("> ").strip().lower()

            if cmd == "exit":
                print("Goodbye!")
                return
            elif cmd == "b":
                break  # back to filter selection
            elif cmd == "n":
                if (page + 1) * PAGE_SIZE < len(filtered_phrases):
                    page += 1
                else:
                    print("Already at last page.")
            elif cmd == "p":
                if page > 0:
                    page -= 1
                else:
                    print("Already at first page.")
            elif cmd.isdigit():
                idx = int(cmd)
                page_phrases, total = paginate_list(filtered_phrases, page, PAGE_SIZE)
                if 1 <= idx <= len(page_phrases):
                    speak(page_phrases[idx - 1]["phrase"])
                else:
                    print("Invalid phrase number.")
            else:
                print("Invalid command.")


if __name__ == "__main__":
    main()
