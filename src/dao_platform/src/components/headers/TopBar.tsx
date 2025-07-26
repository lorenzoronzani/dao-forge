import React from 'react';
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Copy } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import { useAuthentication } from "@/providers/AuthenticationProvider";
import { useNavigate } from "react-router";
import { paths } from "@/constants/paths";

export const TopBar = () => {
    const { isAuthenticated, userPrincipal, login, logout } = useAuthentication();
    const { toast } = useToast();
    const navigate = useNavigate();

    // Permission policy error
    // await navigator.clipboard.writeText(userPrincipal.toText());
    const copyUserPrincipal = () => {
        if (!isAuthenticated) return;

        const text = userPrincipal.toText();

        // Create a temporary textarea element
        const textArea = document.createElement("textarea");
        textArea.value = text;

        // Make it invisible but accessible
        textArea.style.position = "absolute";
        textArea.style.left = "-9999px";
        textArea.style.top = "-9999px";
        textArea.style.opacity = "0";
        textArea.setAttribute('readonly', '');
        textArea.style.pointerEvents = 'none';
        textArea.style.zIndex = '-1';
        textArea.setAttribute('tabindex', '-1');

        document.body.appendChild(textArea);

        try {
            // Focus and select the text
            textArea.focus();
            textArea.select();
            textArea.setSelectionRange(0, 99999); // For mobile devices

            // Execute the copy command
            const successful = document.execCommand('copy');

            if (successful) {
                toast({
                    title: "Copied!",
                    description: "User principal copied to clipboard.",
                    duration: 2000,
                });
            } else {
                throw new Error('execCommand failed');
            }
        } catch (error) {
            toast({
                title: "Error!",
                description: "Failed to copy user principal to clipboard.",
                duration: 2000,
            });
        } finally {
            // Always clean up the temporary element
            document.body.removeChild(textArea);
        }
    };

    return (
        <header className="bg-white shadow-sm">
            <div className="container mx-auto px-4 py-4 flex justify-between items-center">
                <h1 className="text-xl font-bold cursor-pointer hover:text-primary transition-colors" onClick={() => navigate(paths.HOME)}>DAO forge</h1>
                {isAuthenticated ? (
                    <div className="flex items-center gap-4">
                        <Card className="h-10 px-3 flex items-center gap-2">
                            <span className="text-sm font-medium truncate max-w-[200px]">{userPrincipal.toText()}</span>
                            <Button variant="ghost" size="icon" onClick={copyUserPrincipal} className="h-8 w-8 p-0">
                                <Copy className="h-4 w-4" />
                            </Button>
                        </Card>
                        <Button variant="outline" onClick={logout}>Logout</Button>
                    </div>
                ) : (
                    <Button onClick={login}>Login</Button>
                )}
            </div>
        </header>
    )
};

export default TopBar;