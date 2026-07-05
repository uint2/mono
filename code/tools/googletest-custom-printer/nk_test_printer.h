#ifndef _NK_TEST_PRINTER_H
#define _NK_TEST_PRINTER_H 1

#include <gtest/gtest.h>

class NkTestPrinter : public testing::EmptyTestEventListener {
    void OnTestStart(const testing::TestInfo &t) override;
    void OnTestPartResult(const testing::TestPartResult &tpr) override;
    void OnTestEnd(const testing::TestInfo &t) override;
    void OnTestProgramEnd(const testing::UnitTest &u) override;

  private:
    const bool is_atty_{isatty(STDOUT_FILENO) ? true : false};
};

#endif
