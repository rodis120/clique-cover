package pl.rogal.reggraphgen;

import org.apache.commons.cli.*;


public class Main {

    public static void main(String[] args) {
        int port = 8080;

        Option helpOpt = Option.builder("h")
                .desc("Print help")
                .longOpt("help")
                .build();
        Option portOpt = Option.builder("p")
                .hasArg()
                .argName("port")
                .desc("Set port to be used by http server. (Default: "+ port +")")
                .longOpt("port")
                .build();

        Options options = new Options();
        options.addOption(helpOpt);
        options.addOption(portOpt);

        CommandLineParser cliParser = new DefaultParser();
        try {
            CommandLine cli = cliParser.parse(options, args);

            if(cli.hasOption(helpOpt)) {
                printHelp(options);
                return;
            }

            if(cli.hasOption(portOpt)) {
                try {
                    port = Integer.parseInt(cli.getOptionValue(portOpt));
                } catch (NumberFormatException e) {
                    System.err.println("Incorrect value for option: port");
                    printHelp(options);
                    return;
                }
            }
        } catch (ParseException e) {
            printHelp(options);
            return;
        }

        RestApi.startRestApi(port);
    }

    private static void printHelp(Options options) {
        HelpFormatter help = new HelpFormatter();
        help.printHelp("reggraphgen", options);
    }
}