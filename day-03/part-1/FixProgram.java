import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class FixProgram {
	public static void main(String[] args) {
		int total = 0;
		String input = null;

		try {
			input = Files.readString(Paths.get("input.txt"));
		} catch (Exception e) {
			System.err.println("Failed to open file: " + e);
			System.exit(1);
		}

		String regex = "mul\\((\\d+),(\\d+)\\)";
		Pattern pattern = Pattern.compile(regex);
		Matcher matcher = pattern.matcher(input);

		while (matcher.find()) {
			int firstNumber = Integer.parseInt(matcher.group(1));
			int secondNumber = Integer.parseInt(matcher.group(2));
			if (firstNumber > 999 || secondNumber > 999) {
				continue;
			}
			total += firstNumber * secondNumber;
		}

		System.out.println("Total: " + total);
	}
}