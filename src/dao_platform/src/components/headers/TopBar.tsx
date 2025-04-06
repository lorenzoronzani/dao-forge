import React from 'react';
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Copy } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import { useAuthentication } from "@/providers/AuthenticationProvider";

export const TopBar = () => {
    const { isAuthenticated, userPrincipal, login, logout } = useAuthentication();
    const { toast } = useToast();

    const copyUserId = () => {
        if (isAuthenticated) {
            navigator.clipboard.writeText(userPrincipal.toText());
            toast({
                title: "Copied!",
                description: "User ID copied to clipboard.",
                duration: 2000,
            });
        }
    };

    return (
        <div className="w-full h-16 px-4 flex items-center justify-between border-b">
            <div className="text-xl font-bold">DAO forge</div>

            <div className="flex items-center gap-4">
                {isAuthenticated ? (
                    <>
                        <Card className="h-10 px-3 flex items-center gap-2">
                            <span className="text-sm font-medium truncate max-w-[200px]">{userPrincipal.toText()}</span>
                            <Button variant="ghost" size="icon" onClick={copyUserId} className="h-8 w-8 p-0">
                                <Copy className="h-4 w-4" />
                            </Button>
                        </Card>
                        <Button onClick={logout}>Logout</Button>
                    </>
                ) : (
                    <Button onClick={login}>Login</Button>
                )}
            </div>
        </div>
    );
};

export default TopBar;