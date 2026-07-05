#include "nk_test_printer.h"

#define C1 "\e[31m"
#define C2 "\e[32m"
#define C3 "\e[33m"
#define CR "\e[m"

void NkTestPrinter::OnTestStart(const testing::TestInfo &t) {
    std::cout << t.test_suite_name() << '.' << t.name() << " ... ";
    std::flush(std::cout);
}

void NkTestPrinter::OnTestPartResult(const testing::TestPartResult &tpr) {
    if (tpr.skipped() || !tpr.failed()) {
        return;
    }
    std::cout << std::endl << "--" << std::endl;
    if (is_atty_) {
        std::cout << C1 << "Assertion Error in " << CR;
        std::cout << C3 << tpr.file_name() << ':' << tpr.line_number() << CR;
    } else {
        std::cout << "Assertion Error in ";
        std::cout << tpr.file_name() << ':' << tpr.line_number();
    }
    std::cout << std::endl << "--" << std::endl;
    std::cout << tpr.summary();
    std::cout << "--" << std::endl;
}

void NkTestPrinter::OnTestEnd(const testing::TestInfo &t) {
    if (is_atty_) {
        std::cout << C2 << "[ok]" << CR;
    } else {
        std::cout << "[ok]";
    }
    long mils = t.result()->elapsed_time();
    if (mils != 0) {
        std::cout << " (" << mils << "ms)";
    }
    std::cout << std::endl;
}

void NkTestPrinter::OnTestProgramEnd(const testing::UnitTest &u) {
    std::cout << "All " << u.total_test_count() << " test(s) completed."
              << std::endl;
    std::cout << "* passed: " << u.successful_test_count() << std::endl;
    std::cout << "* failed: " << u.failed_test_count() << std::endl;
    std::cout << u.elapsed_time() << "ms elapsed." << std::endl;
}
